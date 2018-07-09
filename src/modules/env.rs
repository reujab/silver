use std::env;
use Segment;

pub fn segment(segment: &mut Segment, args: &[&str]) {
    if args.is_empty() {
        panic!("you must provide an extra argument to the env module");
    }

    segment.value = env::var(args[0]).unwrap_or_default();
}
