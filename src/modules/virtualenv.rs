use std::env;
use std::path::Path;
use Segment;

pub fn segment(segment: &mut Segment, _: &[&str]) {
    segment.value = Path::new(&env::var("VIRTUAL_ENV").unwrap_or_default())
        .file_name()
        .unwrap_or_default()
        .to_str()
        .unwrap()
        .to_owned();
}
