//! Deterministic pair-mode root-cause narrative for `repro why <a> <b>`.

use crate::core::artifact::ExecutionArtifact;
use std::collections::BTreeSet;

fn artifacts_equivalent(a: &ExecutionArtifact, b: &ExecutionArtifact) -> bool {
    if a.run_id == b.run_id {
        return true;
    }
    a.command == b.command
        && a.stdout == b.stdout
        && a.stderr == b.stderr
        && a.exit_code == b.exit_code
        && match (&a.repro_run, &b.repro_run) {
            (Some(ra), Some(rb)) => ra.env == rb.env,
            (None, None) => a.environment_hash() == b.environment_hash(),
            _ => false,
        }
}

/// Build the strict `── AION WHY ──` report for two stored runs (`a` is the first argument, `b` the second).
pub fn explain_pair_why(a: &ExecutionArtifact, b: &ExecutionArtifact) -> String {
    let mut out = String::new();
    out.push_str("── AION WHY ──\n\n");

    if artifacts_equivalent(a, b) {
        out.push_str("NO DIFFERENCE\n");
        return out;
    }

    let stdout_changed = a.stdout != b.stdout;

    let (env_a, env_b) = match (&a.repro_run, &b.repro_run) {
        (Some(ra), Some(rb)) => (&ra.env, &rb.env),
        _ => {
            out.push_str("NO ROOT CAUSE FOUND\n");
            if stdout_changed {
                out.push_str("\nEFFECT:\n  stdout changed\n");
            }
            return out;
        }
    };

    let keys: BTreeSet<String> = env_a.keys().chain(env_b.keys()).cloned().collect();
    let mut changed: Vec<(String, String, String)> = Vec::new();
    for k in keys {
        let va = env_a.get(&k).cloned().unwrap_or_default();
        let vb = env_b.get(&k).cloned().unwrap_or_default();
        if va != vb {
            changed.push((k, va, vb));
        }
    }

    if changed.is_empty() {
        if stdout_changed {
            out.push_str("NO ROOT CAUSE FOUND\n\nEFFECT:\n  stdout changed\n");
        } else {
            out.push_str("NO ROOT CAUSE FOUND\n");
        }
        return out;
    }

    if !stdout_changed {
        out.push_str("NO ROOT CAUSE FOUND\n\n");
        out.push_str("(environment changed but captured stdout is identical)\n");
        return out;
    }

    if changed.len() == 1 {
        let (k, before, after) = &changed[0];
        out.push_str("ROOT CAUSE:\n");
        out.push_str(&format!("  {k} changed\n\n"));
        out.push_str("VALUES:\n");
        out.push_str(&format!("  before: {before}\n"));
        out.push_str(&format!("  after:  {after}\n\n"));
        out.push_str("EFFECT:\n");
        out.push_str("  stdout changed\n\n");
        out.push_str("CAUSAL CHAIN:\n");
        out.push_str(&format!("  {k} → stdout\n"));
        return out;
    }

    out.push_str("MULTI-CAUSE:\n");
    for (k, before, after) in &changed {
        out.push_str(&format!(
            "  - {k} changed (before: {before} / after: {after})\n"
        ));
    }
    out.push('\n');
    out.push_str("EFFECT:\n");
    out.push_str("  stdout changed\n\n");
    out.push_str("CAUSAL CHAIN:\n");
    let labels: Vec<&str> = changed.iter().map(|(k, _, _)| k.as_str()).collect();
    out.push_str("  ");
    out.push_str(&labels.join(", "));
    out.push_str(" → stdout\n");
    out
}
