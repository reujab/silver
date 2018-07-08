use std::env;

pub fn separator() -> String {
    env::var("SILVER_SEPARATOR").unwrap_or("\u{e0b0}".to_owned())
}

pub fn thin_separator() -> String {
    env::var("SILVER_THIN_SEPARATOR").unwrap_or("\u{e0b1}".to_owned())
}
