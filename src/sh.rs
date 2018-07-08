use regex::Regex;

pub fn escape_background(shell: &str, color: &str) -> String {
    let hex = Regex::new(r"^[a-f\d]{6}$").unwrap();

    match shell {
        "zsh" => {
            if hex.is_match(&color) {
                format!("%{{\x1b[48;2;{}m%}}", escape_hex(color))
            } else {
                format!("%K{{{}}}", color)
            }
        }
        // TODO: other shells
        _ => panic!(),
    }
}

pub fn escape_foreground(shell: &str, color: &str) -> String {
    let hex = Regex::new(r"^[a-f\d]{6}$").unwrap();

    match shell {
        "zsh" => {
            if hex.is_match(&color) {
                format!("%{{\x1b[38;2;{}m%}}", escape_hex(color))
            } else {
                format!("%F{{{}}}", color)
            }
        }
        // TODO: other shells
        _ => panic!(),
    }
}

fn escape_hex(color: &str) -> String {
    format!(
        "{};{};{}",
        u8::from_str_radix(&color[0..2], 16).unwrap(),
        u8::from_str_radix(&color[2..4], 16).unwrap(),
        u8::from_str_radix(&color[4..6], 16).unwrap(),
    )
}

pub fn reset_colors(shell: &str) {
    match shell {
        "zsh" => print!("%{{%f%}}"),
        "bash" => print!("\\[\x1b[0m\\]"),
        _ => print!("\x1b[0m"),
    }
}
