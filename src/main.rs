mod cli;
mod config;
mod icons;
mod modules;
mod print;
mod sh;

use cli::*;
use once_cell::sync::{Lazy, OnceCell};
use std::{
    env,
    path::{Path, PathBuf},
};
use sysinfo::{get_current_pid, ProcessExt, RefreshKind, System, SystemExt};

static CONFIG_PATH: OnceCell<PathBuf> = OnceCell::new();

static CONFIG: Lazy<config::Config> = Lazy::new(|| {
    if let Some(path) = CONFIG_PATH.get() {
        confy::load_path(path)
    } else {
        confy::load("silver")
    }
    .expect("Failed to read config")
});

const INIT_BASH: &str = include_str!("init.bash");
const INIT_PS1: &str = include_str!("init.ps1");
const INIT_ION: &str = include_str!("init.ion");

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

/// Helper function for trimming a String in place. Derived from
/// https://users.rust-lang.org/t/trim-string-in-place/15809/9
fn replace_with_subslice<F>(this: &mut String, f: F)
where
    F: FnOnce(&str) -> &str,
{
    let original_len = this.len();
    let new_slice: &str = f(this);
    let start = (new_slice.as_ptr() as usize).wrapping_sub(this.as_ptr() as usize);
    if original_len < start {
        this.clear();
        return;
    }

    let len = new_slice.len();
    if start != 0 {
        this.drain(..start);
    }
    this.truncate(len);
}

/// Identify the type of shell in use.
fn get_shell(opt: &cli::Silver) -> String {
    let mut shell: String = if let Some(ref s) = opt.shell {
        // If --shell was given on the command line, use that.
        s.clone()
    } else if let Ok(s) = env::var("SILVER_SHELL") {
        // For backward compatibility with 1.1 and earlier,
        // use the value of the SILVER_SHELL environment variable.
        s
    } else {
        // Use the name of the parent process, if we can.
        // Minimize the amount of information loaded by sysinfo.
        // FIXME: Proper error handling, not all these unwraps.
        let mut sys = System::new_with_specifics(RefreshKind::new());
        // It'd be nice if either the stdlib or sysinfo exposed
        // getppid() directly, but they don't.
        let mypid = get_current_pid().unwrap();
        sys.refresh_process(mypid);
        let parentpid = sys.get_process(mypid).unwrap().parent().unwrap();
        sys.refresh_process(parentpid);

        sys.get_process(parentpid).unwrap().name().to_string()
    };

    // For Windows compatibility, lowercase the shell's name
    // ("BASH.EXE" is the same as "bash.exe").
    shell.make_ascii_lowercase();

    // Remove any leading or trailing whitespace from `shell`.
    // Remove anything up to and including the last '/' or '\\',
    //   in case the shell was identified by absolute path.
    // Remove a trailing ".exe" if present, for Windows compatibility.
    // Remove a leading "-" if present; on Unix this is a flag indicating
    //   a login shell (see login(1) and sh(1)), not part of the name.
    // These are done unconditionally, not just when we are using
    // the name of the parent process to identify the shell, because
    // the installation instructions for older versions of silver
    // said to set SILVER_SHELL to "$0" without trimming anything
    // from it.
    replace_with_subslice(&mut shell, |s| {
        let s = s.trim();
        let s = s.rsplit(&['/', '\\'][..]).next().unwrap_or(s);
        let s = s.strip_prefix('-').unwrap_or(s);
        let s = s.strip_suffix(".exe").unwrap_or(s);
        s
    });

    shell
}

fn main() {
    let opt = cli::Silver::from_args();

    if let Some(ref path) = opt.config {
        let path = Path::new(path.as_str()).canonicalize().unwrap();
        CONFIG_PATH.set(path).unwrap()
    }

    let _shell = get_shell(&opt);
    let shell = _shell.as_str();

    match opt.cmd {
        Command::Init => {
            let script = match shell {
                "bash" => INIT_BASH,
                "powershell" | "pwsh" => INIT_PS1,
                "ion" => INIT_ION,
                _ => {
                    use std::process::exit;
                    eprintln!("silver: unknown shell: \"{}\".", shell);
                    eprintln!("silver: supported shells: bash, ion, powershell");
                    exit(1);
                }
            };
            let script = script.replace("@SILVER_SHELL@", shell);
            let script = if let Some(path) = CONFIG_PATH.get() {
                script.replace(
                    "silver",
                    format!("silver --config {}", path.display()).as_str(),
                )
            } else {
                script
            };
            print!("{}", script);
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
            print!(" ")
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
