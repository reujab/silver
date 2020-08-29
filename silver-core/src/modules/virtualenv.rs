use crate::Segment;
use std::{env, path::Path};

/// Needs exporting `VIRTUAL_ENV_DISABLE_PROMPT` with some value
pub struct VirtualEnv;

impl Segment for VirtualEnv {
    fn draw(&self, _: &dyn crate::IconProvider) -> Option<String> {
        Some(
            Path::new(&env::var("VIRTUAL_ENV").ok()?)
                .file_name()?
                .to_str()?
                .to_owned(),
        )
    }
}
