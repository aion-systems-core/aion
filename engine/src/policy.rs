//! Resolve named [`PolicyProfile`](aion_core::PolicyProfile).

use aion_core::PolicyProfile;

pub fn resolve(name: &str) -> PolicyProfile {
    match name {
        "stage" => PolicyProfile::stage(),
        "prod" => PolicyProfile::prod(),
        _ => PolicyProfile::dev(),
    }
}

pub fn net_policy_for(p: &PolicyProfile) -> aion_core::NetPolicy {
    aion_core::NetPolicy {
        deny_outbound: p.no_network,
        allow_loopback: true,
    }
}
