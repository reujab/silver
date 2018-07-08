use Segment;

mod dir;

pub fn handle(module: &str, segment: &mut Segment, args: &[&str]) {
    match module {
        "dir" => dir::segment(segment, args),
        _ => panic!("unknown module, {}", module),
    }
}
