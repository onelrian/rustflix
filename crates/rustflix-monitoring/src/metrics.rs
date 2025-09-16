//! Metrics collection and reporting

use rustflix_core::{Result, RustFlixError};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use tracing::{info, debug};

/// Metrics collector for system performance data
#[derive(Debug, Clone)]
pub struct MetricsCollector {
    metrics: Arc<RwLock<HashMap<String, Metric>>>,
}

/// Individual metric data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
    pub name: String,
    pub value: f64,
    pub timestamp: DateTime<Utc>,
    pub labels: HashMap<String, String>,
    pub metric_type: MetricType,
}

/// Types of metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricType {
    Counter,
    Gauge,
    Histogram,
    Summary,
}

impl MetricsCollector {
    /// Create a new metrics collector
    pub fn new() -> Result<Self> {
        Ok(Self {
            metrics: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Record a counter metric
    pub async fn increment_counter(&self, name: &str, value: f64, labels: HashMap<String, String>) -> Result<()> {
        let mut metrics = self.metrics.write().await;
        
        let metric = Metric {
            name: name.to_string(),
            value,
            timestamp: Utc::now(),
            labels,
            metric_type: MetricType::Counter,
        };
        
        metrics.insert(name.to_string(), metric);
        debug!("Recorded counter metric: {} = {}", name, value);
        Ok(())
    }

    /// Record a gauge metric
    pub async fn set_gauge(&self, name: &str, value: f64, labels: HashMap<String, String>) -> Result<()> {
        let mut metrics = self.metrics.write().await;
        
        let metric = Metric {
            name: name.to_string(),
            value,
            timestamp: Utc::now(),
            labels,
            metric_type: MetricType::Gauge,
        };
        
        metrics.insert(name.to_string(), metric);
        debug!("Set gauge metric: {} = {}", name, value);
        Ok(())
    }

    /// Get all metrics
    pub async fn get_metrics(&self) -> Result<Vec<Metric>> {
        let metrics = self.metrics.read().await;
        Ok(metrics.values().cloned().collect())
    }

    /// Export metrics in Prometheus format
    pub async fn export_prometheus(&self) -> Result<String> {
        let metrics = self.metrics.read().await;
        let mut output = String::new();
        
        for metric in metrics.values() {
            let labels_str = if metric.labels.is_empty() {
                String::new()
            } else {
                let labels: Vec<String> = metric.labels
                    .iter()
                    .map(|(k, v)| format!("{}=\"{}\"", k, v))
                    .collect();
                format!("{{{}}}", labels.join(","))
            };
            
            output.push_str(&format!("{}{} {}\n", metric.name, labels_str, metric.value));
        }
        
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_metrics_collector_creation() {
        let collector = MetricsCollector::new();
        assert!(collector.is_ok());
    }

    #[tokio::test]
    async fn test_increment_counter() {
        let collector = MetricsCollector::new().unwrap();
        let labels = HashMap::new();
        
        let result = collector.increment_counter("test_counter", 1.0, labels).await;
        assert!(result.is_ok());
        
        let metrics = collector.get_metrics().await.unwrap();
        assert_eq!(metrics.len(), 1);
        assert_eq!(metrics[0].name, "test_counter");
        assert_eq!(metrics[0].value, 1.0);
    }

    #[tokio::test]
    async fn test_export_prometheus() {
        let collector = MetricsCollector::new().unwrap();
        let labels = HashMap::new();
        
        collector.increment_counter("test_metric", 42.0, labels).await.unwrap();
        
        let prometheus_output = collector.export_prometheus().await.unwrap();
        assert!(prometheus_output.contains("test_metric 42"));
    }
}
