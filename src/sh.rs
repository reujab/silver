use regex::Regex;

lazy_static! {
    // global regex constant
    static ref HEX: Regex = Regex::new(r"^[a-f\d]{6}$").unwrap();
}

fn code(color: &str, prefix: &str, light_prefix: &str) -> Option<String> {
    let (color, prefix) = if color.starts_with("light") {
        (&color[5..], light_prefix)
    } else {
        (color, prefix)
    };
    match color {
        "none" => Some("0".to_owned()),
        "black" => Some(format!("{}0", prefix)),
        "red" => Some(format!("{}1", prefix)),
        "green" => Some(format!("{}2", prefix)),
        "yellow" => Some(format!("{}3", prefix)),
        "blue" => Some(format!("{}4", prefix)),
        "magenta" => Some(format!("{}5", prefix)),
        "cyan" => Some(format!("{}6", prefix)),
        "white" => Some(format!("{}7", prefix)),
        _ => None,
    }
}

fn escape(
    color: &str,
    prefix: &str,
    light_prefix: &str,
    before_color: &str,
    after_color: &str,
) -> String {
    match code(&color, &prefix, &light_prefix) {
        // 16 colors
        Some(code) => format!("{}\x1b[{}m{}", before_color, code, after_color),
        None => match color.parse::<u8>() {
            // 256 colors
            Ok(_) => format!(
                "{}\x1b[{}8;5;{}m{}",
                before_color, prefix, color, after_color,
            ),
            Err(_) => {
                // 24-bit color
                if HEX.is_match(&color) {
                    format!(
                        "{}\x1b[{}8;2;{}m{}",
                        before_color,
                        prefix,
                        escape_hex(color),
                        after_color
                    )
                } else {
                    panic!("invalid background color: {}", color)
                }
            }
        },
    }
}

pub fn escape_background(shell: &str, color: &str) -> String {
    match shell {
        "zsh" => escape(color, "4", "10", "%{", "%}"),
        "bash" => escape(color, "4", "10", "\\[", "\\]"),
        _ => escape(color, "4", "10", "", ""),
    }
}

pub fn escape_foreground(shell: &str, color: &str) -> String {
    match shell {
        "zsh" => escape(color, "3", "9", "%{", "%}"),
        "bash" => escape(color, "3", "9", "\\[", "\\]"),
        _ => escape(color, "3", "9", "", ""),
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
