use crate::Segment;
use std::env;

pub struct Toolbox;

impl Segment for Toolbox {
    fn draw(&self, icons: &dyn crate::IconProvider) -> Option<String> {
        env::var("TOOLBOX_PATH").ok()?;
        icons.get("toolbox").map(|c| c.to_string())
    }
}
