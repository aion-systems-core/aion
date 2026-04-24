use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum DeterminismAxis {
    Os,
    Arch,
    Locale,
    Timezone,
    Seed,
    Env,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeterminismTarget {
    pub os: String,
    pub arch: String,
    pub locale: String,
    pub timezone: String,
    pub seed: u64,
    pub env_profile: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeterminismResult {
    pub target: DeterminismTarget,
    pub status: String,
    pub code: String,
    pub context: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeterminismMatrix {
    pub axes: Vec<DeterminismAxis>,
    pub targets: Vec<DeterminismTarget>,
    pub results: Vec<DeterminismResult>,
}

fn supported_target(t: &DeterminismTarget) -> bool {
    matches!(t.os.as_str(), "linux" | "macos" | "windows")
        && matches!(t.arch.as_str(), "x64" | "arm64")
        && t.locale == "en_US.UTF-8"
        && t.timezone == "UTC"
        && t.seed == 42
        && t.env_profile == "frozen"
}

pub fn evaluate_determinism_matrix(mut targets: Vec<DeterminismTarget>) -> DeterminismMatrix {
    targets.sort_by(|a, b| {
        (
            a.os.clone(),
            a.arch.clone(),
            a.locale.clone(),
            a.timezone.clone(),
            a.seed,
            a.env_profile.clone(),
        )
            .cmp(&(
                b.os.clone(),
                b.arch.clone(),
                b.locale.clone(),
                b.timezone.clone(),
                b.seed,
                b.env_profile.clone(),
            ))
    });
    let results = targets
        .iter()
        .map(|t| {
            let ok = supported_target(t);
            DeterminismResult {
                target: t.clone(),
                status: if ok { "ok".into() } else { "error".into() },
                code: if ok {
                    "AION_OK".into()
                } else {
                    "determinism:matrix_target_failed".into()
                },
                context: "determinism_matrix.evaluate".into(),
                cause: if ok {
                    None
                } else {
                    Some("unsupported_or_nondeterministic_target".into())
                },
            }
        })
        .collect();
    DeterminismMatrix {
        axes: vec![
            DeterminismAxis::Os,
            DeterminismAxis::Arch,
            DeterminismAxis::Locale,
            DeterminismAxis::Timezone,
            DeterminismAxis::Seed,
            DeterminismAxis::Env,
        ],
        targets,
        results,
    }
}

#[cfg(test)]
mod tests {
    use super::{evaluate_determinism_matrix, DeterminismTarget};

    #[test]
    fn matrix_serialization_deterministic() {
        let m = evaluate_determinism_matrix(vec![DeterminismTarget {
            os: "linux".into(),
            arch: "x64".into(),
            locale: "en_US.UTF-8".into(),
            timezone: "UTC".into(),
            seed: 42,
            env_profile: "frozen".into(),
        }]);
        assert_eq!(
            serde_json::to_string(&m).unwrap(),
            serde_json::to_string(&m).unwrap()
        );
    }

    #[test]
    fn negative_seed_locale_timezone() {
        let m = evaluate_determinism_matrix(vec![DeterminismTarget {
            os: "linux".into(),
            arch: "x64".into(),
            locale: "de_DE.UTF-8".into(),
            timezone: "Europe/Berlin".into(),
            seed: 43,
            env_profile: "frozen".into(),
        }]);
        assert_eq!(m.results[0].status, "error");
    }
}
