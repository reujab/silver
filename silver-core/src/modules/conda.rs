use super::Segment;
use std::env;

pub struct Conda;
impl Segment for Conda {
    fn draw(&self, _: &dyn crate::IconProvider) -> Option<String> {
        env::var("CONDA_PROMPT_MODIFIER")
            .map(|s| s.trim().to_string())
            .ok()
    }
}
