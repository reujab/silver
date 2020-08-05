mod config;
mod icons;
mod modules;
mod print;
mod sh;

use clap::App;
use clap::AppSettings;
use once_cell::sync::Lazy;
use std::path::Path;
use sysinfo::{get_current_pid, ProcessExt, System, SystemExt};

static CONFIG: Lazy<config::Config> =
    Lazy::new(|| confy::load("silver").expect("Failed to read config"));

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
    let shell = parent.name();

    let matches = App::new("silver")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .about("a cross-shell customizable powerline-like prompt with icons")
        .after_help("https://github.com/reujab/silver/wiki")
        .subcommand(
            clap::SubCommand::with_name("init").about("Initializes the shell for use of silver"),
        )
        .subcommand(
            clap::SubCommand::with_name("lprint")
                .alias("print")
                .about("Prints the left prompt with the specified modules"),
        )
        .subcommand(
            clap::SubCommand::with_name("rprint")
                .about("Prints the right prompt with the specified modules"),
        )
        .get_matches();

    match matches.subcommand_name().unwrap() {
        "init" => match Path::new(&shell).to_str().unwrap() {
            "bash" => print!("{}", include_str!("init.bash")),
            "zsh" => print!("{}", include_str!("init.zsh")),
            "fish" => print!("{}", include_str!("init.fish")),
            "powershell" | "pwsh" => print!("{}", include_str!("init.powershell")),
            "ion" => print!(include_str!("init.ion")),
            _ => panic!(
                "unknown shell: \"{}\". Supported shells: bash, zsh, fish, powershell",
                shell
            ),
        },
        "lprint" => {
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
        "rprint" => print::prompt(&shell, &CONFIG.right, |_, (p, c, _)| {
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
        _ => panic!(),
    }
}
