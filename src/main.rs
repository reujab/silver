mod cli;
mod config;
mod icons;
mod modules;
mod print;
mod sh;

use cli::*;
use once_cell::sync::{Lazy, OnceCell};
use std::path::{Path, PathBuf};
use sysinfo::{get_current_pid, System};

static CONFIG_PATH: OnceCell<PathBuf> = OnceCell::new();

static CONFIG: Lazy<config::Config> = Lazy::new(|| {
    match CONFIG_PATH.get() {
        Some(path) => confy::load_path(path),
        None => confy::load("silver", None),
    }
    .expect("Problem loading or serializing default config")
});

#[derive(Clone, Debug)]
pub struct Segment {
    background: String,
    foreground: String,
    value:      String,
}

impl Default for Segment {
    fn default() -> Self {
        Self {
            background: "none".to_owned(),
            foreground: "none".to_owned(),
            value:      String::new(),
        }
    }
}

fn main() {
    let sys = System::new_all();
    let process = sys.process(get_current_pid().unwrap()).unwrap();
    let parent = sys.process(process.parent().unwrap()).unwrap();
    let shell = std::env::var("SILVER_SHELL").unwrap_or_else(|_| {
        let shell = parent.name().trim();
        let shell = shell.strip_suffix(".exe").unwrap_or(shell);
        shell.strip_prefix('-').unwrap_or(shell).to_owned()
    });

    let opt = cli::Silver::parse();

    if let Some(path) = opt.config {
        let path = Path::new(path.as_str()).canonicalize().unwrap();
        CONFIG_PATH.set(path).unwrap()
    }

    match opt.cmd {
        Command::Init => {
            print!(
                "{}",
                match shell.as_str() {
                    "bash" => include_str!("init.bash"),
                    "powershell" | "pwsh" => include_str!("init.ps1"),
                    "ion" => include_str!("init.ion"),
                    _ =>
                        panic!(
                            "unknown shell: \"{}\". Supported shells: bash, ion, powershell",
                            shell
                        ),
                }
                .replace(
                    "silver",
                    format!(
                        "silver{}",
                        if let Some(path) = CONFIG_PATH.get() {
                            format!(" --config {}", path.display())
                        } else {
                            String::new()
                        }
                    )
                    .as_str()
                )
            )
        }
        Command::Lprint => {
            print::prompt(&shell, &CONFIG.left, |_, (_, c, n)| {
                vec![
                    (
                        c.background.to_owned(),
                        c.foreground.to_owned(),
                        format!(" {} ", c.value),
                    ),
                    if n.background == c.background {
                        (
                            c.background.to_owned(),
                            c.foreground.to_owned(),
                            CONFIG.separator.left.thin.to_owned(),
                        )
                    } else {
                        (
                            n.background.to_owned(),
                            c.background.to_owned(),
                            CONFIG.separator.left.thick.to_owned(),
                        )
                    },
                ]
            });
            print!(" ");
        }

        Command::Rprint => {
            print::prompt(&shell, &CONFIG.right, |_, (p, c, _)| {
                vec![
                    if p.background == c.background {
                        (
                            c.background.to_owned(),
                            c.foreground.to_owned(),
                            CONFIG.separator.right.thin.to_owned(),
                        )
                    } else {
                        (
                            p.background.to_owned(),
                            c.background.to_owned(),
                            CONFIG.separator.right.thick.to_owned(),
                        )
                    },
                    (
                        c.background.to_owned(),
                        c.foreground.to_owned(),
                        format!(" {} ", c.value),
                    ),
                ]
            })
        }
    }
}
