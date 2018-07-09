use std::env;
use Segment;

pub fn segment(segment: &mut Segment, args: &[&str]) {
    if args.len() == 0 {
        panic!("you must provide an extra argument to the env module");
    }

    segment.value = env::var(args[0]).unwrap_or(String::new());
}
