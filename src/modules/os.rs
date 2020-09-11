#[cfg(target_os = "linux")]
use ini::Ini;

use crate::{icons, Segment};

#[cfg(target_os = "macos")]
pub fn segment(segment: &mut Segment, _: &[&str]) {
    segment.value = icons::get("apple");
}

#[cfg(any(target_os = "openbsd", target_os = "freebsd", target_os = "netbsd"))]
pub fn segment(segment: &mut Segment, _: &[&str]) {
    segment.value = icons::get("bsd");
}

#[cfg(target_os = "linux")]
pub fn segment(segment: &mut Segment, _: &[&str]) {
    let release = Ini::load_from_file("/etc/os-release").unwrap();
    segment.value = match release.general_section()["ID"].as_ref() {
        "arch" => icons::get("arch"),
        "centos" => icons::get("centOS"),
        "debian" => icons::get("debian"),
        "fedora" => icons::get("fedora"),
        "linuxmint" => icons::get("mint"),
        "suse" | "opensuse" => icons::get("SUSE"),
        "ubuntu" => icons::get("ubuntu"),
        "elementary" => icons::get("elementary"),
        _ => icons::get("linux"),
    }
}

#[cfg(target_os = "windows")]
pub fn segment(segment: &mut Segment, _: &[&str]) {
    segment.value = icons::get("windows");
}
