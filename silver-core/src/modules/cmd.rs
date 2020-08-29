use super::Segment;
use crate::IconProvider;
use std::process::Command;

pub struct Cmd {
    cmd: Vec<String>,
}

impl Segment for Cmd {
    fn draw(&self, _: &dyn IconProvider) -> Option<String> {
        String::from_utf8(
            Command::new(self.cmd.get(0)?)
                .args(self.cmd.get(1..).unwrap_or_default())
                .output()
                .ok()?
                .stdout,
        )
        .ok()
    }
}
