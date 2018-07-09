use icons;
use std::env;
use Segment;

pub fn segment(segment: &mut Segment, _: &[&str]) {
    let mut wd = env::current_dir().unwrap().to_str().unwrap().to_owned();

    // process aliases
    let mut aliases = match env::var("SILVER_DIR_ALIASES") {
        Ok(var) => var.split(":")
            .map(|alias| alias.to_owned())
            .collect::<Vec<String>>(),
        Err(_) => vec![],
    };
    if aliases.len() % 2 != 0 {
        panic!("invalid $SILVER_DIR_ALIASES");
    }
    // default home alias
    match env::home_dir() {
        Some(home) => {
            aliases.push(home.to_str().unwrap().to_owned());
            aliases.push(icons::get("home"));
        }
        None => {}
    }
    for i in 0..aliases.len() / 2 {
        let dir = &aliases[i * 2];
        if format!("{}/", wd).starts_with(&format!("{}/", dir)) {
            let icon = &aliases[i * 2 + 1];
            wd = format!("{}{}", icon, &wd[dir.len()..]);
            break;
        }
    }

    segment.value = wd;
}
