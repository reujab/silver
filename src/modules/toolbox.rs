use icons;
use std::env;
use Segment;

pub fn segment(segment: &mut Segment, _: &[&str]) {
    if env::var("TOOLBOX_PATH").is_err() {
        return;
    }

    segment.value = icons::get("toolbox");
}
