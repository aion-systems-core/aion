//! Rank simple textual hypotheses (stable ordering).

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hypothesis {
    pub score: u8,
    pub text: String,
}

pub fn rank(mut items: Vec<Hypothesis>) -> Vec<Hypothesis> {
    items.sort_by(|a, b| b.score.cmp(&a.score).then_with(|| a.text.cmp(&b.text)));
    items
}
