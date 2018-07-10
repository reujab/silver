#[cfg(unix)]
use libc;
#[cfg(unix)]
use std::ffi::CString;

use icons;
use std::env;
use users;
use Segment;

pub fn segment(segment: &mut Segment, _: &[&str]) {
    if users::get_current_uid() == 0 {
        segment.value += &icons::get("root");
    }

    // considering both macOS and BSD are POSIX, i'm assuming this will work on macOS and BSD, but i haven't checked
    // https://linux.die.net/man/2/access
    #[cfg(unix)]
    unsafe {
        let wd = CString::new(".").unwrap();
        if libc::access(wd.as_ptr(), libc::W_OK) != libc::F_OK {
            segment.value += &icons::get("readonly");
        }
    }

    if let Ok(code) = env::var("code") {
        if code != "0" {
            segment.value += &icons::get("failed");
        }
    }

    if let Ok(jobs) = env::var("jobs") {
        if let Ok(jobs_count) = usize::from_str_radix(jobs.trim(), 10) {
            segment.value += &icons::get("job").repeat(jobs_count);
        }
    }
}
