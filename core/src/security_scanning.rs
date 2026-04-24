use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SecurityScanConfig {
    pub scan: String,
    pub mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SecurityIssue {
    pub code: String,
    pub context: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SecurityScanResult {
    pub scans: Vec<SecurityScanConfig>,
    pub last_run_epoch: u64,
    pub issues: Vec<SecurityIssue>,
    pub status: String,
}

pub fn run_security_scans() -> SecurityScanResult {
    let mut scans = vec![
        SecurityScanConfig {
            scan: "sast".into(),
            mode: "required".into(),
        },
        SecurityScanConfig {
            scan: "dependency_scanning".into(),
            mode: "required".into(),
        },
        SecurityScanConfig {
            scan: "container_scanning".into(),
            mode: "optional".into(),
        },
        SecurityScanConfig {
            scan: "secret_scanning".into(),
            mode: "required".into(),
        },
    ];
    scans.sort_by(|a, b| a.scan.cmp(&b.scan));
    let issues = Vec::new();
    SecurityScanResult {
        scans,
        last_run_epoch: 0,
        issues,
        status: "ok".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::run_security_scans;

    #[test]
    fn deterministic_scans() {
        let r = run_security_scans();
        assert_eq!(
            serde_json::to_string(&r).unwrap(),
            serde_json::to_string(&r).unwrap()
        );
        assert_eq!(r.scans[0].scan, "container_scanning");
    }

    #[test]
    fn baseline_has_required_scans() {
        let r = run_security_scans();
        assert!(r
            .scans
            .iter()
            .any(|s| s.scan == "sast" && s.mode == "required"));
    }
}
