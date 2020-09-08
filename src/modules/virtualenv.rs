use crate::Segment;
use std::env;
use std::path::Path;

pub fn segment(segment: &mut Segment, _: &[&str]) {
    segment.value = Path::new(&env::var("VIRTUAL_ENV").unwrap_or_default())
        .iter()
        .last()
        .unwrap_or_default()
        .to_str()
        .unwrap()
        .to_owned();
}
