use Segment;

mod dir;
mod os;

pub fn handle(module: &str, segment: &mut Segment, args: &[&str]) {
    match module {
        "os" => os::segment(segment, args),
        "dir" => dir::segment(segment, args),
        _ => panic!("unknown module, {}", module),
    }
}
