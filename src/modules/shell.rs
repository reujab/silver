use crate::Segment;
use sysinfo::{get_current_pid, ProcessExt, System, SystemExt};

pub fn segment(segment: &mut Segment, _: &[&str]) {
    let sys = System::new_all();
    segment.value = sys
        .process(
            sys.process(get_current_pid().unwrap())
                .unwrap()
                .parent()
                .unwrap(),
        )
        .unwrap()
        .name()
        .to_string();
}
