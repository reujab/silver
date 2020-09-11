use crate::Segment;

#[cfg(target_os = "windows")]
mod users {
    use std::{env, ffi::OsString};

    pub fn get_current_username() -> Option<OsString> {
        env::var("USERNAME").ok().map(|s| s.into())
    }
}

pub fn segment(segment: &mut Segment, _: &[&str]) {
    segment.value = format!(
        "{}@{}",
        users::get_current_username()
            .unwrap()
            .into_string()
            .unwrap(),
        hostname::get().unwrap().to_string_lossy()
    )
}
