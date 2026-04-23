#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionEnvelope {
    pub seed: u64,
    pub env_hash: String,
    pub kernel_version: String,
    pub policy_version: String,
    pub hardware_profile: String,
}

impl ExecutionEnvelope {
    pub fn from_env() -> Self {
        Self {
            seed: 0,
            env_hash: String::new(),
            kernel_version: "0".to_string(),
            policy_version: "0".to_string(),
            hardware_profile: "default".to_string(),
        }
    }
}
