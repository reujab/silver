use crate::Segment;
use std::env;

pub fn segment(segment: &mut Segment, _: &[&str]) {
    segment.value = env::var("CONDA_PROMPT_MODIFIER")
        .unwrap_or_default()
        .trim()
        .to_string();
}
