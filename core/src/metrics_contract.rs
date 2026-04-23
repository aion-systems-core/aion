use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum MetricNamespace {
    Kernel,
    Cli,
    Replay,
    Policy,
    Evidence,
    Ops,
    Security,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MetricType {
    Counter,
    Gauge,
    Histogram,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MetricStatus {
    Defined,
    Missing,
    Deprecated,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MetricDefinition {
    pub name: String,
    pub namespace: MetricNamespace,
    pub metric_type: MetricType,
    pub status: MetricStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MetricsContract {
    pub metrics: Vec<MetricDefinition>,
    pub status: String,
}

pub fn evaluate_metrics_contract(mut metrics: Vec<MetricDefinition>) -> MetricsContract {
    metrics.sort_by(|a, b| a.name.cmp(&b.name));
    let has_missing = metrics.iter().any(|m| m.status == MetricStatus::Missing);
    MetricsContract {
        metrics,
        status: if has_missing { "error" } else { "ok" }.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn missing_metric_negative() {
        let c = evaluate_metrics_contract(vec![MetricDefinition {
            name: "aion_replay_total".into(),
            namespace: MetricNamespace::Replay,
            metric_type: MetricType::Counter,
            status: MetricStatus::Missing,
        }]);
        assert_eq!(c.status, "error");
    }
}

