use regex::Regex;

lazy_static! {
    // global regex constant
    static ref HEX: Regex = Regex::new(r"^[a-f\d]{6}$").unwrap();
}

fn code(color: &str, prefix: &str) -> Option<String> {
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

fn bash_color(color: &str, prefix: &str) -> String {
    match code(&color, &prefix) {
        // 16 colors
        Some(code) => format!("\\[\x1b[{}m\\]", code),
        None => match u8::from_str_radix(&color, 10) {
            // 256 colors
            Ok(_) => format!("\\[\x1b[{}8;5;{}m\\]", prefix, color),
            Err(_) => {
                // 24-bit color
                if HEX.is_match(&color) {
                    format!("\\[\x1b[{}8;2;{}m\\]", prefix, escape_hex(color))
                } else {
                    panic!("invalid background color: {}", color)
                }
            }
        },
    }
}

pub fn escape_background(shell: &str, color: &str) -> String {
    match shell {
        "zsh" => {
            if HEX.is_match(&color) {
                format!("%{{\x1b[48;2;{}m%}}", escape_hex(color))
            } else {
                format!("%K{{{}}}", color)
            }
        }
        "bash" => bash_color(color, "4"),
        _ => panic!(),
    }
}

pub fn escape_foreground(shell: &str, color: &str) -> String {
    match shell {
        "zsh" => {
            if HEX.is_match(&color) {
                format!("%{{\x1b[38;2;{}m%}}", escape_hex(color))
            } else {
                format!("%F{{{}}}", color)
            }
        }
        "bash" => bash_color(color, "3"),
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
