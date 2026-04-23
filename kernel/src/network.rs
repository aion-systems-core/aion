//! Network policy placeholder (real sandboxing is out of kernel v2 scope).

use aion_core::NetPolicy;

pub fn apply_net_policy_stub(policy: &NetPolicy) -> Result<(), String> {
    if policy.deny_outbound && !policy.allow_loopback {
        return Err("network policy forbids all traffic (stub)".into());
    }
    Ok(())
}
