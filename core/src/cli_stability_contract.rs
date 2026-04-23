use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CliCommandSurface {
    pub command: String,
    pub flags: Vec<CliFlag>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CliFlag {
    pub name: String,
    pub change_type: CliChangeType,
    pub deprecation_warning: Option<CliDeprecationWarning>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CliChangeType {
    Compatible,
    Deprecated,
    Breaking,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CliDeprecationWarning {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CliStabilityContract {
    pub surfaces: Vec<CliCommandSurface>,
    pub status: String,
}

pub fn evaluate_cli_stability_contract(mut contract: CliStabilityContract) -> CliStabilityContract {
    contract.surfaces.sort_by(|a, b| a.command.cmp(&b.command));
    for s in &mut contract.surfaces {
        s.flags.sort_by(|a, b| a.name.cmp(&b.name));
    }
    let missing_warning = contract.surfaces.iter().any(|s| {
        s.flags.iter().any(|f| {
            (f.change_type == CliChangeType::Deprecated || f.change_type == CliChangeType::Breaking)
                && f.deprecation_warning.is_none()
        })
    });
    contract.status = if missing_warning { "error".into() } else { "ok".into() };
    contract
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cli_flag_without_warning_negative() {
        let c = evaluate_cli_stability_contract(CliStabilityContract {
            surfaces: vec![CliCommandSurface {
                command: "aion test".into(),
                flags: vec![CliFlag {
                    name: "--legacy".into(),
                    change_type: CliChangeType::Deprecated,
                    deprecation_warning: None,
                }],
            }],
            status: String::new(),
        });
        assert_eq!(c.status, "error");
    }
}

