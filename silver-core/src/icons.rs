use std::collections::HashMap;

pub trait IconProvider {
    fn get(&self, name: &str) -> Option<char>;
    fn repeat(&self, name: &str, count: usize) -> Option<String> {
        let icon = self.get(name)?.to_string();
        Some(if count > 3 {
            format!("{}{}", icon, count)
        } else {
            icon.repeat(count)
        })
    }
}

pub struct NerdFonts;
pub struct Unicode {
    pub use_apple_logo: bool,
}
pub struct ASCII;
pub struct Custom<'a>(HashMap<&'a str, char>);

impl IconProvider for Custom<'_> {
    fn get(&self, name: &str) -> Option<char> {
        self.0.get(name).copied()
    }
}
impl<T> IconProvider for T
where
    T: Iterator<Item = Box<dyn IconProvider>> + Clone,
{
    fn get(&self, name: &str) -> Option<char> {
        for i in self.clone() {
            if let Some(icon) = i.get(name) {
                return Some(icon);
            }
        }
        None
    }
}

impl IconProvider for NerdFonts {
    fn get(&self, name: &str) -> Option<char> {
        match name {
            // OS
            "alpine" => Some('\u{f300}'),
            "amazon" => Some('\u{f270}'),
            "arch" => Some('\u{f303}'),
            "bsd" => Some('\u{f30c}'),
            "centos" => Some('\u{f304}'),
            "debian" => Some('\u{f306}'),
            "fedora" => Some('\u{f30a}'),
            "manjaro" => Some('\u{f312}'),
            "suse" => Some('\u{f314}'),
            "redhat" => Some('\u{f316}'),
            "ubuntu" => Some('\u{f31b}'),
            "linux" => Some('\u{f31a}'),
            "android" => Some('\u{f17b}'),
            "apple" => Some('\u{f179}'),
            "windows" => Some('\u{f17a}'),
            // Git
            "github" => Some('\u{f09b}'),
            "gitlab" => Some('\u{f296}'),
            "bitbucket" => Some('\u{f171}'),
            "azure" => Some('\u{fd03}'),
            "git" => Some('\u{f418}'),
            "stash" => Some('\u{f01c}'),
            "ahead" => Some('\u{f148}'),
            "behind" => Some('\u{f149}'),
            "modified" => Some('\u{f111}'),
            "staged" => Some('\u{f067}'),
            // Other
            "toolbox" => Some('\u{f0ad}'),
            "root" => Some('\u{e00a}'),
            "readonly" => Some('\u{f023}'),
            "failed" => Some('\u{e009}'),
            "job" => Some('\u{e615}'),
            "package" => Some('\u{f187}'),
            "rss" => Some('\u{f09e}'),
            "home" => Some('\u{f015}'),
            _ => None,
        }
    }
}

impl IconProvider for Unicode {
    fn get(&self, name: &str) -> Option<char> {
        match name {
            "apple" => {
                if self.use_apple_logo {
                    Some('\u{f8ff}')
                } else {
                    Some('\u{1f34e}')
                }
            }
            "linux" => Some('\u{1f427}'),
            "bsd" => Some('\u{1f608}'),
            "windows" => Some('\u{1f5a5}'),
            "root" => Some('\u{26a1}'),
            "readonly" => Some('\u{1f512}'),
            "failed" => Some('\u{2757}'),
            "job" => Some('\u{2699}'),
            "package" => Some('\u{1f4e6}'),
            "home" => Some('\u{1f3e0}'),
            "stash" => Some('\u{1f4e5}'),
            "ahead" => Some('\u{2191}'),
            "behind" => Some('\u{2193}'),
            "modified" => Some('\u{25cf}'),
            "staged" => Some('\u{271a}'),
            "toolbox" => Some('\u{1f527}'),
            _ => None,
        }
    }
}

impl IconProvider for ASCII {
    fn get(&self, name: &str) -> Option<char> {
        match name {
            "root" => Some('#'),
            "readonly" => Some('@'),
            "failed" => Some('!'),
            "job" => Some('&'),
            "rss" => Some('*'),
            "home" => Some('~'),
            "stash" => Some('#'),
            "ahead" => Some('>'),
            "behind" => Some('<'),
            "modified" => Some('*'),
            "staged" => Some('+'),
            "toolbox" => Some('*'),
            _ => None,
        }
    }
}
