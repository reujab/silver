use Segment;

mod dir;

use modules::dir::dir_segment;

pub fn handle(module: &str, segment: &mut Segment, args: &[&str]) {
    match module {
        "dir" => dir_segment(segment, args),
        _ => panic!("unknown module, {}", module),
    }
}
