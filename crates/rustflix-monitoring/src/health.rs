//! Health check functionality

use rustflix_core::{Result, RustFlixError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use tracing::{info, warn, debug};

/// Health checker for system components
pub struct HealthChecker {
    checks: HashMap<String, Box<dyn HealthCheck + Send + Sync>>,
}

/// Health check status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: Status,
    pub checks: HashMap<String, CheckResult>,
    pub timestamp: DateTime<Utc>,
}

/// Overall health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Status {
    Healthy,
    Degraded,
    Unhealthy,
}

/// Individual check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    pub status: Status,
    pub message: String,
    pub duration_ms: u64,
}

/// Health check trait
pub trait HealthCheck {
    fn name(&self) -> &str;
    fn check(&self) -> Result<CheckResult>;
}

impl HealthChecker {
    /// Create a new health checker
    pub fn new() -> Self {
        Self {
            checks: HashMap::new(),
        }
    }

    /// Clone the health checker (manual implementation since trait objects can't be cloned)
    pub fn clone_checker(&self) -> Self {
        Self {
            checks: HashMap::new(), // Start with empty checks - they need to be re-registered
        }
    }

    /// Add a health check
    pub fn add_check(&mut self, check: Box<dyn HealthCheck + Send + Sync>) {
        self.checks.insert(check.name().to_string(), check);
    }

    /// Run all health checks
    pub async fn check_health(&self) -> Result<HealthStatus> {
        let mut check_results = HashMap::new();
        let mut overall_status = Status::Healthy;

        for (name, check) in &self.checks {
            match check.check() {
                Ok(result) => {
                    match result.status {
                        Status::Degraded if matches!(overall_status, Status::Healthy) => {
                            overall_status = Status::Degraded;
                        }
                        Status::Unhealthy => {
                            overall_status = Status::Unhealthy;
                        }
                        _ => {}
                    }
                    check_results.insert(name.clone(), result);
                }
                Err(e) => {
                    warn!("Health check '{}' failed: {}", name, e);
                    overall_status = Status::Unhealthy;
                    check_results.insert(name.clone(), CheckResult {
                        status: Status::Unhealthy,
                        message: format!("Check failed: {}", e),
                        duration_ms: 0,
                    });
                }
            }
        }

        Ok(HealthStatus {
            status: overall_status,
            checks: check_results,
            timestamp: Utc::now(),
        })
    }
}

/// Database health check
pub struct DatabaseHealthCheck;

impl HealthCheck for DatabaseHealthCheck {
    fn name(&self) -> &str {
        "database"
    }

    fn check(&self) -> Result<CheckResult> {
        // Placeholder implementation
        Ok(CheckResult {
            status: Status::Healthy,
            message: "Database connection OK".to_string(),
            duration_ms: 5,
        })
    }
}

/// Redis health check
pub struct RedisHealthCheck;

impl HealthCheck for RedisHealthCheck {
    fn name(&self) -> &str {
        "redis"
    }

    fn check(&self) -> Result<CheckResult> {
        // Placeholder implementation
        Ok(CheckResult {
            status: Status::Healthy,
            message: "Redis connection OK".to_string(),
            duration_ms: 3,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_checker_creation() {
        let checker = HealthChecker::new();
        // HealthChecker::new() returns Self, not Result
        let _checker = checker;
    }

    #[tokio::test]
    async fn test_add_health_check() {
        let mut checker = HealthChecker::new();
        checker.add_check(Box::new(DatabaseHealthCheck));
        
        let status = checker.check_health().await.unwrap();
        assert!(matches!(status.status, Status::Healthy));
        assert_eq!(status.checks.len(), 1);
    }
}
