#[cfg(unix)]
use std::ffi::CString;

use crate::{icons, Segment};
use std::env;

#[cfg(target_os = "windows")]
fn is_root() -> bool {
    use std::mem;
    use winapi::{
        ctypes::c_void,
        shared::minwindef::{DWORD, TRUE},
        um::{
            handleapi::{CloseHandle, INVALID_HANDLE_VALUE},
            processthreadsapi::{GetCurrentProcess, OpenProcessToken},
            securitybaseapi::GetTokenInformation,
            winnt::{TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY},
        },
    };

    let mut token = INVALID_HANDLE_VALUE;
    let mut elevated = false;

    if unsafe { OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) == TRUE } {
        let mut elevation: TOKEN_ELEVATION = unsafe { mem::zeroed() };
        let mut size = mem::size_of::<TOKEN_ELEVATION>() as DWORD;
        if unsafe {
            GetTokenInformation(
                token,
                TokenElevation,
                &mut elevation as *mut TOKEN_ELEVATION as *mut c_void,
                size,
                &mut size,
            ) == TRUE
        } {
            elevated = elevation.TokenIsElevated != 0;
        }
    }

    if token != INVALID_HANDLE_VALUE {
        unsafe { CloseHandle(token) };
    }

    elevated
}

#[cfg(not(target_os = "windows"))]
fn is_root() -> bool {
    users::get_current_uid() == 0
}

pub fn segment(segment: &mut Segment, _: &[&str]) {
    if is_root() {
        segment.value += &icons::get("root");
    }

    // considering both macOS and BSD are POSIX, i'm assuming this will work on
    // macOS and BSD, but i haven't checked https://linux.die.net/man/2/access
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
        if let Ok(jobs_count) = jobs.trim().parse() {
            segment.value += &icons::get("job").repeat(jobs_count);
        }
    }
}
