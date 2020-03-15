use Segment;
use icons;
use std::env;

pub fn segment(segment: &mut Segment, _: &[&str]) {
    if let Err(_) = env::var("TOOLBOX_PATH") {
        return;
    }

    segment.value = icons::get("toolbox");
}
