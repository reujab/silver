use crate::Segment;

pub struct User;

impl Segment for User {
    fn draw(&self, icons: &dyn crate::IconProvider) -> Option<String> {
        Some(format!(
            "{is_root}{username}@{hostname}",
            is_root = if is_root::is_root() {
                icons.get("root")
            } else {
                None
            }
            .unwrap_or_default()
            .to_string(),
            username = whoami::username(),
            hostname = whoami::hostname()
        ))
    }
}
