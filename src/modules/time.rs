use chrono;
use chrono::Timelike;
use Segment;

pub fn segment(segment: &mut Segment, _: &[&str]) {
    let now = chrono::offset::Local::now();
    let (pm, hour) = now.hour12();
    let minute = now.minute();
    segment.value = format!("{}:{:02}{}", hour, minute, if pm { "PM" } else { "AM" });
}
