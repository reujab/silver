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
        if libc::access(CString::new(".").unwrap().as_ptr(), libc::W_OK) != libc::F_OK {
            segment.value += &icons::get("readonly");
        }
    }

    match env::var("code") {
        Ok(code) => if code != "0" {
            segment.value += &icons::get("failed")
        },
        Err(_) => {}
    }

    match env::var("jobs") {
        Ok(jobs) => {
            segment.value += &icons::get("job").repeat(usize::from_str_radix(&jobs, 10).unwrap())
        }
        Err(_) => {}
    }
}
