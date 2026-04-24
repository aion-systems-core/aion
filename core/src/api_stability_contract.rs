use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ApiSurface {
    CliJsonApi,
    ConfigSchema,
    DoctorOutput,
    ContractsApi,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ApiChangeType {
    Compatible,
    Deprecated,
    Breaking,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ApiDeprecationNotice {
    pub since_version: String,
    pub sunset_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ApiStabilityContract {
    pub surface: ApiSurface,
    pub change_type: ApiChangeType,
    pub deprecation_notice: Option<ApiDeprecationNotice>,
    pub status: String,
}

pub fn evaluate_api_stability_contract(mut contract: ApiStabilityContract) -> ApiStabilityContract {
    contract.status = match contract.change_type {
        ApiChangeType::Compatible => "ok".into(),
        ApiChangeType::Deprecated => {
            if contract.deprecation_notice.is_some() {
                "warn".into()
            } else {
                "error".into()
            }
        }
        ApiChangeType::Breaking => {
            if contract.deprecation_notice.is_some() {
                "warn".into()
            } else {
                "error".into()
            }
        }
    };
    contract
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn api_change_without_notice_negative() {
        let c = evaluate_api_stability_contract(ApiStabilityContract {
            surface: ApiSurface::DoctorOutput,
            change_type: ApiChangeType::Deprecated,
            deprecation_notice: None,
            status: String::new(),
        });
        assert_eq!(c.status, "error");
    }
}
