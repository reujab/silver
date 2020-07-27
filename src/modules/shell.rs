use crate::Segment;
use sysinfo::{get_current_pid, ProcessExt, System, SystemExt};

pub fn segment(segment: &mut Segment, _: &[&str]) {
    let sys = System::new_all();
    segment.value = sys
        .get_process(
            sys.get_process(get_current_pid().unwrap())
                .unwrap()
                .parent()
                .unwrap(),
        )
        .unwrap()
        .name()
        .to_string();
}
