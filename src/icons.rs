use std::env;

pub fn separator() -> String {
    env::var("SILVER_SEPARATOR").unwrap_or_else(|_| "\u{e0b0}".to_owned())
}

pub fn thin_separator() -> String {
    env::var("SILVER_THIN_SEPARATOR").unwrap_or_else(|_| "\u{e0b1}".to_owned())
}

pub fn get(id: &str) -> String {
    let icon = env::var(format!("SILVER_ICON_{}", id.to_lowercase()));
    match icon {
        Ok(icon) => icon,
        Err(_) => match env::var("SILVER_ICONS")
            .unwrap_or_else(|_| "nerd".to_owned())
            .as_str()
        {
            "nerd" => match id {
                "apple" => "\u{f179}",      // Font Awesome; apple
                "arch" => "\u{f303}",       // Font Linux
                "centOS" => "\u{f304}",     // Font Linux
                "debian" => "\u{f306}",     // Font Linux
                "fedora" => "\u{f30a}",     // Font Linux
                "mint" => "\u{f30e}",       // Font Linux
                "SUSE" => "\u{f314}",       // Font Linux
                "ubuntu" => "\u{f31b}",     // Font Linux
                "elementary" => "\u{f309}", // Font Linux
                "linux" => "\u{f31a}",      // Font Linux
                "bsd" => "\u{f30c}",        // Font Linux
                "root" => "\u{e00a}",       // Pomicons; lightning
                "readonly" => "\u{f023}",   // Font Awesome; lock
                "failed" => "\u{e009}",     // Pomicons; exclamation
                "job" => "\u{e615}",        // Seti UI
                "package" => "\u{f187}",    // Font Awesome; archive
                "rss" => "\u{f09e}",        // Font Awesome; rss
                "home" => "\u{f015}",       // Font Awesome; home
                "github" => "\u{f09b}",     // Font Awesome; github
                "gitlab" => "\u{f296}",     // Font Awesome; gitlab
                "bitbucket" => "\u{f171}",  // Font Awesome; bitbucket
                "git" => "\u{e0a0}",        // Powerline
                "stash" => "\u{f01c}",      // Font Awesome; inbox
                "ahead" => "\u{f148}",      // Font Awesome; level-up
                "behind" => "\u{f149}",     // Font Awesome; level-down
                "modified" => "\u{f111}",   // Unicode
                "staged" => "\u{f067}",     // Unicode
                _ => "",
            }
            .to_owned(),
            "unicode" => match id {
                // TODO: test if it's possible to use \uf8ff on an Apple machine
                "apple" => "\u{1f34e}",    // Emoji; red apple
                "linux" => "\u{1f427}",    // Emoji; penguin
                "bsd" => "\u{1f608}",      // Emoji; smiling face with horns
                "root" => "\u{26a1}",      // Emoji; high voltage
                "readonly" => "\u{1f512}", // Emoji; locked
                "failed" => "\u{2757}",    // Emoji; exclamation mark
                "job" => "\u{2699}",       // Emoji; gear
                "package" => "\u{1f4e6}",  // Emoji; package
                "rss" => "*",
                "home" => "\u{1f3e0}",    // Emoji; house
                "stash" => "\u{1f4e5}",   // Emoji; inbox tray
                "ahead" => "\u{2191}",    // Unicode
                "behind" => "\u{2193}",   // Unicode
                "modified" => "\u{25cf}", // Unicode
                "staged" => "\u{271a}",   // Unicode
                _ => "",
            }
            .to_owned(),
            "ascii" => match id {
                "root" => "#",
                "readonly" => "@",
                "failed" => "!",
                "job" => "&",
                "rss" => "*",
                "home" => "~",
                "stash" => "#",
                "ahead" => ">",
                "behind" => "<",
                "modified" => "*",
                "staged" => "+",
                _ => "",
            }
            .to_owned(),
            _ => panic!("unknown $SILVER_ICONS"),
        },
    }
}

pub fn repeat(icon: &str, n: usize) -> String {
    let icon = get(icon);

    if n > 5 {
        format!("{}{}", icon, n)
    } else {
        icon.repeat(n)
    }
}
