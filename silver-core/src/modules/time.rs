use crate::Segment;

pub struct Time {
    pub am_pm: Option<bool>,
}

impl Segment for Time {
    fn draw(&self, _: &dyn crate::IconProvider) -> Option<String> {
        let now = chrono::Local::now().time();
        Some(
            match self.am_pm {
                None => now.format("%X"),
                Some(false) => now.format("%T"),
                Some(true) => now.format("%r"),
            }
            .to_string(),
        )
    }
}
