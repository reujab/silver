mod cli;
mod config;
mod icons;
mod modules;
mod print;
mod sh;

use cli::*;
use once_cell::sync::{Lazy, OnceCell};
use std::path::{Path, PathBuf};
use sysinfo::{get_current_pid, ProcessExt, System, SystemExt};

static CONFIG_PATH: OnceCell<PathBuf> = OnceCell::new();

static CONFIG: Lazy<config::Config> = Lazy::new(|| {
    if let Some(path) = CONFIG_PATH.get() {
        confy::load_path(path)
    } else {
        confy::load("silver")
    }
    .expect("Failed to read config")
});

#[derive(Clone, Debug)]
pub struct Segment {
    background: String,
    foreground: String,
    value: String,
}

impl Default for Segment {
    fn default() -> Self {
        Self {
            background: "none".to_owned(),
            foreground: "none".to_owned(),
            value: String::new(),
        }
    }
}

fn main() {
    let sys = System::new_all();
    let process = sys.get_process(get_current_pid().unwrap()).unwrap();
    let parent = sys.get_process(process.parent().unwrap()).unwrap();
    let shell = parent.name().trim();
    let opt = cli::Silver::from_args();
    if let Some(path) = opt.config {
        let path = Path::new(path.as_str()).canonicalize().unwrap();
        CONFIG_PATH.set(path).unwrap()
    }

    match opt.cmd {
        Command::Init => print!(
            "{}",
            match shell {
                "bash" => include_str!("init.bash"),
                "powershell" | "pwsh" | "powershell.exe" | "pwsh.exe" => include_str!("init.ps1"),
                // "codelldb.exe" => include_str!("init.ps1"), //only used when debugging with codelldb on windows
                "ion" => include_str!("init.ion"),
                _ => panic!(
                    "unknown shell: \"{}\". Supported shells: bash, ion, powershell",
                    shell
                ),
            }
            .replace(
                "silver",
                format!(
                    "silver.exe {}",
                    if let Some(path) = CONFIG_PATH.get() {
                        let path_canon = path.canonicalize().expect("invalid path");
                        format!(
                            "-ArgumentList @('--config','{}','lprint')",
                            adjust_canonicalization(path_canon)
                        )
                    } else {
                        "-ArgumentList @('lprint')".to_string()
                    }
                )
                .as_str()
            )
        ),
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
            print!(" ")
        }

        Command::Rprint => print::prompt(&shell, &CONFIG.right, |_, (p, c, _)| {
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
        }),
    }
}

#[cfg(not(target_os = "windows"))]
fn adjust_canonicalization<P: AsRef<Path>>(p: P) -> String {
    p.as_ref().display().to_string()
}

#[cfg(target_os = "windows")]
fn adjust_canonicalization<P: AsRef<Path>>(p: P) -> String {
    const VERBATIM_PREFIX: &str = r#"\\?\"#;
    let p = p.as_ref().display().to_string();
    if p.starts_with(VERBATIM_PREFIX) {
        p[VERBATIM_PREFIX.len()..].to_string()
    } else {
        p
    }
}
