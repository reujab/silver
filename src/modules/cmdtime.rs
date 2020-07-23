use crate::Segment;

use std::env;
use std::time::Duration;

pub fn segment(segment: &mut Segment, _: &[&str]) {
    if let Ok(cmdtime) = env::var("cmdtime") {
        let elapsed = Duration::from_millis(cmdtime.parse().unwrap());
        let threshold = match env::var("SILVER_CMDTIME_THRESHOLD") {
            Ok(threshold) => {
                humantime::parse_duration(&threshold).expect("invalid $SILVER_CMDTIME_THRESHOLD")
            }
            Err(_) => Duration::new(0, 0),
        };

        if elapsed >= threshold {
            segment.value = humantime::format_duration(elapsed).to_string();
        }
    }
}
