use crate::Segment;
use sysinfo::{get_current_pid, ProcessExt, RefreshKind, System, SystemExt};

pub struct Shell {
    sys: System,
}

impl Shell {
    #[must_use]
    pub fn new() -> Self {
        Self {
            sys: System::new_with_specifics(RefreshKind::new().with_processes()),
        }
    }
}

impl Segment for Shell {
    fn draw(&self, _: &dyn crate::IconProvider) -> Option<String> {
        Some(
            self.sys
                .get_process(self.sys.get_process(get_current_pid().unwrap())?.parent()?)?
                .name()
                .to_string(),
        )
    }
}
