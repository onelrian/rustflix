//! Alert management functionality

use rustflix_core::{Result, RustFlixError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use tracing::{info, warn, debug};

/// Alert manager for system notifications
#[derive(Debug, Clone)]
pub struct AlertManager {
    rules: HashMap<String, AlertRule>,
    active_alerts: HashMap<Uuid, Alert>,
}

/// Alert rule configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    pub name: String,
    pub condition: String,
    pub threshold: f64,
    pub severity: AlertSeverity,
    pub enabled: bool,
}

/// Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
}

/// Active alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub id: Uuid,
    pub rule_name: String,
    pub message: String,
    pub severity: AlertSeverity,
    pub timestamp: DateTime<Utc>,
    pub resolved: bool,
}

impl AlertManager {
    /// Create a new alert manager
    pub fn new() -> Result<Self> {
        Ok(Self {
            rules: HashMap::new(),
            active_alerts: HashMap::new(),
        })
    }

    /// Add alert rule
    pub fn add_rule(&mut self, rule: AlertRule) {
        info!("Adding alert rule: {}", rule.name);
        self.rules.insert(rule.name.clone(), rule);
    }

    /// Trigger alert
    pub fn trigger_alert(&mut self, rule_name: &str, message: &str) -> Result<Uuid> {
        let rule = self.rules.get(rule_name)
            .ok_or_else(|| RustFlixError::not_found("alert_rule", rule_name))?;

        let alert = Alert {
            id: Uuid::new_v4(),
            rule_name: rule_name.to_string(),
            message: message.to_string(),
            severity: rule.severity.clone(),
            timestamp: Utc::now(),
            resolved: false,
        };

        let alert_id = alert.id;
        self.active_alerts.insert(alert_id, alert);
        
        warn!("Alert triggered: {} - {}", rule_name, message);
        Ok(alert_id)
    }

    /// Resolve alert
    pub fn resolve_alert(&mut self, alert_id: Uuid) -> Result<()> {
        if let Some(alert) = self.active_alerts.get_mut(&alert_id) {
            alert.resolved = true;
            info!("Alert resolved: {}", alert.rule_name);
        }
        Ok(())
    }

    /// Get active alerts
    pub fn get_active_alerts(&self) -> Vec<&Alert> {
        self.active_alerts
            .values()
            .filter(|alert| !alert.resolved)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alert_manager_creation() {
        let manager = AlertManager::new();
        assert!(manager.is_ok());
    }

    #[test]
    fn test_add_rule_and_trigger_alert() {
        let mut manager = AlertManager::new().unwrap();
        
        let rule = AlertRule {
            name: "high_cpu".to_string(),
            condition: "cpu_usage > 80".to_string(),
            threshold: 80.0,
            severity: AlertSeverity::Warning,
            enabled: true,
        };
        
        manager.add_rule(rule);
        
        let alert_id = manager.trigger_alert("high_cpu", "CPU usage is 85%").unwrap();
        assert!(manager.active_alerts.contains_key(&alert_id));
        
        let active_alerts = manager.get_active_alerts();
        assert_eq!(active_alerts.len(), 1);
    }
}
