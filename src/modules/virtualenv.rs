use std::env;
use std::ffi::OsStr;
use std::path::Path;
use Segment;

pub fn segment(segment: &mut Segment, _: &[&str]) {
    segment.value = Path::new(&env::var("VIRTUAL_ENV").unwrap_or(String::new()))
        .file_name()
        .unwrap_or(OsStr::new(""))
        .to_str()
        .unwrap()
        .to_owned();
}
