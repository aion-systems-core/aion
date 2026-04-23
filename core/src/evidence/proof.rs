//! Chain proof: rolling digest binds each step to the previous.

use super::chain::{EvidenceChain, EvidenceRecord};
use super::hashing::sha256_hex;
use crate::error::{code, line};
use crate::{DeterminismProfile, PolicyProfile, RunResult};

fn roll(parent: &str, leaf: &str) -> String {
    sha256_hex(format!("{parent}|{leaf}").as_bytes())
}

/// Build a minimal auditable chain from a run + policy + determinism snapshot.
pub fn seal_run(
    run: &RunResult,
    policy: &PolicyProfile,
    det: &DeterminismProfile,
) -> EvidenceChain {
    let mut records = Vec::new();
    let mut prev_roll: Option<String> = None;
    let items: [(&str, Vec<u8>); 4] = [
        ("run", serde_json::to_vec(run).unwrap_or_default()),
        ("policy", serde_json::to_vec(policy).unwrap_or_default()),
        ("determinism", serde_json::to_vec(det).unwrap_or_default()),
        (
            "artifact_schema",
            run.schema_version.to_string().into_bytes(),
        ),
    ];
    for (i, (kind, bytes)) in items.into_iter().enumerate() {
        let leaf = sha256_hex(&bytes);
        let parent_label = prev_roll.as_deref().unwrap_or("GENESIS");
        let digest = roll(parent_label, &leaf);
        records.push(EvidenceRecord {
            seq: i as u32,
            kind: kind.into(),
            leaf_digest: leaf,
            payload_digest: digest.clone(),
            parent_digest: prev_roll.clone(),
        });
        prev_roll = Some(digest);
    }
    EvidenceChain {
        run_id: run.run_id.clone(),
        records,
        formal_replay_invariant_ok: None,
        cross_machine_replay_ok: None,
    }
}

pub fn verify_linear(chain: &EvidenceChain) -> Result<(), String> {
    if chain.records.is_empty() {
        return Err(line(
            code::EVIDENCE_ANCHOR,
            "verify_linear",
            "evidence:anchor_missing",
        ));
    }
    let root = chain.root_digest();
    if root.is_empty() || root == sha256_hex(b"empty") {
        return Err(line(
            code::EVIDENCE_ANCHOR,
            "verify_linear",
            "evidence:anchor_missing",
        ));
    }
    let mut prev_roll: Option<String> = None;
    for r in &chain.records {
        let parent_label = prev_roll.as_deref().unwrap_or("GENESIS");
        let expect = roll(parent_label, &r.leaf_digest);
        if expect != r.payload_digest {
            return Err(line(
                code::EVIDENCE_HASH,
                "verify_linear",
                &format!("evidence:hash_mismatch:seq:{}", r.seq),
            ));
        }
        if r.parent_digest != prev_roll {
            return Err(line(
                code::EVIDENCE_ANCHOR,
                "verify_linear",
                &format!("evidence:anchor_mismatch:seq:{}", r.seq),
            ));
        }
        prev_roll = Some(r.payload_digest.clone());
    }
    if chain.formal_replay_invariant_ok == Some(false)
        || chain.cross_machine_replay_ok == Some(false)
    {
        return Err(line(
            code::EVIDENCE_ANCHOR,
            "verify_linear",
            "evidence:replay_anchor_mismatch",
        ));
    }
    Ok(())
}
