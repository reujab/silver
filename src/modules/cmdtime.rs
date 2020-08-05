use crate::Segment;
use crate::CONFIG;

use std::env;
use std::time::Duration;

pub fn segment(segment: &mut Segment, _: &[&str]) {
    if let Ok(cmdtime) = env::var("cmdtime") {
        let elapsed = Duration::from_millis(cmdtime.parse().unwrap());
        let threshold = CONFIG.cmdtime_threshold;

        if elapsed >= threshold {
            segment.value = humantime::format_duration(elapsed).to_string();
        }
    }
}
