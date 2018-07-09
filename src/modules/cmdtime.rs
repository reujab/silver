use humantime;
use std::env;
use std::time::Duration;
use Segment;

pub fn segment(segment: &mut Segment, _: &[&str]) {
    match env::var("cmdtime") {
        Ok(cmdtime) => {
            let elapsed = Duration::from_millis(u64::from_str_radix(&cmdtime, 10).unwrap());
            let threshold = match env::var("SILVER_CMDTIME_THRESHOLD") {
                Ok(threshold) => humantime::parse_duration(&threshold)
                    .expect("invalid $SILVER_CMDTIME_THRESHOLD"),
                Err(_) => Duration::new(0, 0),
            };

            if elapsed >= threshold {
                segment.value = humantime::format_duration(elapsed).to_string();
            }
        }
        Err(_) => {}
    }
}
