use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MigrationStep {
    pub id: String,
    pub scope: String,
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpgradePath {
    pub from_version: String,
    pub to_version: String,
    pub steps: Vec<MigrationStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DowngradePath {
    pub from_version: String,
    pub to_version: String,
    pub steps: Vec<MigrationStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MigrationRisk {
    pub id: String,
    pub level: String,
    pub mitigation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpgradeMigrationContract {
    pub upgrade_paths: Vec<UpgradePath>,
    pub downgrade_paths: Vec<DowngradePath>,
    pub risks: Vec<MigrationRisk>,
    pub status: String,
}

pub fn evaluate_upgrade_migration_contract(
    mut upgrade_paths: Vec<UpgradePath>,
    mut downgrade_paths: Vec<DowngradePath>,
    mut risks: Vec<MigrationRisk>,
) -> UpgradeMigrationContract {
    upgrade_paths.sort_by(|a, b| a.to_version.cmp(&b.to_version));
    downgrade_paths.sort_by(|a, b| a.to_version.cmp(&b.to_version));
    for p in &mut upgrade_paths {
        p.steps.sort_by(|a, b| a.id.cmp(&b.id));
    }
    for p in &mut downgrade_paths {
        p.steps.sort_by(|a, b| a.id.cmp(&b.id));
    }
    risks.sort_by(|a, b| a.id.cmp(&b.id));
    let status = if upgrade_paths.iter().any(|p| p.steps.is_empty()) {
        "error"
    } else {
        "ok"
    };
    UpgradeMigrationContract {
        upgrade_paths,
        downgrade_paths,
        risks,
        status: status.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upgrade_negative_without_migration_steps() {
        let c = evaluate_upgrade_migration_contract(
            vec![UpgradePath {
                from_version: "1.0.0".into(),
                to_version: "1.1.0".into(),
                steps: vec![],
            }],
            vec![],
            vec![],
        );
        assert_eq!(c.status, "error");
    }

    #[test]
    fn upgrade_positive_with_steps() {
        let c = evaluate_upgrade_migration_contract(
            vec![UpgradePath {
                from_version: "1.0.0".into(),
                to_version: "1.1.0".into(),
                steps: vec![MigrationStep {
                    id: "01".into(),
                    scope: "contracts".into(),
                    action: "migrate_snapshot_schema".into(),
                }],
            }],
            vec![DowngradePath {
                from_version: "1.1.0".into(),
                to_version: "1.0.0".into(),
                steps: vec![MigrationStep {
                    id: "01".into(),
                    scope: "contracts".into(),
                    action: "restore_snapshot_schema".into(),
                }],
            }],
            vec![MigrationRisk {
                id: "risk1".into(),
                level: "medium".into(),
                mitigation: "preflight_validation".into(),
            }],
        );
        assert_eq!(c.status, "ok");
    }
}

