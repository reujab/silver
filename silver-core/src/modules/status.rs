use crate::Segment;
use std::env;

pub struct Code {
    show_code: bool,
}

pub struct Jobs;

impl Segment for Jobs {
    fn draw(&self, icons: &dyn crate::IconProvider) -> Option<String> {
        icons.repeat("job", env::var("jobs").ok()?.trim().parse().ok()?)
    }
}

impl Segment for Code {
    fn draw(&self, icons: &dyn crate::IconProvider) -> Option<String> {
        match env::var("code").ok()?.parse().ok()? {
            0 => icons.get("ok").map(|c| c.to_string()),
            code => {
                if self.show_code {
                    Some(format!(
                        "{icon}{code}",
                        icon = icons.get("failure").unwrap_or_default(),
                        code = code
                    ))
                } else {
                    icons.get("failure").map(|c| c.to_string())
                }
            }
        }
    }
}
