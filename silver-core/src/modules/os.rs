use super::Segment;
#[cfg(target_os = "linux")]
use os_info::Type::*;

pub struct OS;
impl Segment for OS {
    #[cfg(any(target_os = "openbsd", target_os = "freebsd", target_os = "netbsd"))]
    fn draw(&self, icons: &dyn crate::IconProvider) -> Option<String> {
        icons.get("bsd").map(|c| c.to_string())
    }

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    fn draw(&self, icons: &dyn crate::IconProvider) -> Option<String> {
        icons.get("apple").map(|c| c.to_string())
    }

    #[cfg(target_os = "windows")]
    fn draw(&self, icons: &dyn crate::IconProvider) -> Option<String> {
        icons.get("windows").map(|c| c.to_string())
    }

    #[cfg(target_os = "android")]
    fn draw(&self, icons: &dyn crate::IconProvider) -> Option<String> {
        icons.get("android").map(|c| c.to_string())
    }

    #[cfg(target_os = "linux")]
    #[deny(unused_variables)]
    fn draw(&self, icons: &dyn crate::IconProvider) -> Option<String> {
        match os_info::get().os_type() {
            Alpine => icons.get("alpine"),
            Amazon => icons.get("amazon"),
            Arch => icons.get("arch"),
            Centos => icons.get("centos"),
            Debian => icons.get("debian"),
            EndeavourOS => icons.get("endeavouros"),
            Fedora => icons.get("fedora"),
            Manjaro => icons.get("manjaro"),
            SUSE | openSUSE => icons.get("suse"),
            OracleLinux => icons.get("oracle"),
            Pop => icons.get("popos"),
            Redhat | RedHatEnterprise => icons.get("redhat"),
            Solus => icons.get("solus"),
            Ubuntu => icons.get("ubuntu"),
            _ => None,
        }
        .or_else(|| icons.get("linux"))
        .map(|c| c.to_string())
    }
}
