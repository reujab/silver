use chrono;
use chrono::Timelike;
use Segment;

pub fn segment(segment: &mut Segment, _: &[&str]) {
    let now = chrono::offset::Local::now();
    let (am, hour) = now.hour12();
    let minute = now.minute();
    segment.value = format!("{}:{}{}", hour, minute, if am { "AM" } else { "PM" })
}
