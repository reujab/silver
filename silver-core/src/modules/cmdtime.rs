use super::Segment;
use crate::IconProvider;
use std::time::Duration;

pub struct Cmdtime {
    pub threshold: Duration,
    pub get_time:  Box<dyn Fn() -> Option<Duration>>,
}
impl Segment for Cmdtime {
    fn draw(&self, _: &dyn IconProvider) -> Option<String> {
        let time = (self.get_time)()?;
        if time >= self.threshold {
            Some(humantime::format_duration(time).to_string())
        } else {
            None
        }
    }
}
