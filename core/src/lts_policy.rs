use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum LtsChannel {
    Lts12,
    Lts24,
    Stable,
    Edge,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SupportWindow {
    pub months: u64,
    pub starts_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EolPolicy {
    pub status: String,
    pub eol_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LtsPolicy {
    pub channel: LtsChannel,
    pub support_window: Option<SupportWindow>,
    pub eol_policy: EolPolicy,
    pub status: String,
}

pub fn evaluate_lts_policy(
    channel: LtsChannel,
    support_window: Option<SupportWindow>,
    eol_policy: EolPolicy,
) -> LtsPolicy {
    let status = if support_window.is_none() {
        "error"
    } else {
        "ok"
    };
    LtsPolicy {
        channel,
        support_window,
        eol_policy,
        status: status.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lts_missing_window_negative() {
        let p = evaluate_lts_policy(
            LtsChannel::Lts12,
            None,
            EolPolicy {
                status: "supported".into(),
                eol_date: "2027-01-01".into(),
            },
        );
        assert_eq!(p.status, "error");
    }
}
