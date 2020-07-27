mod icons;
mod modules;
mod print;
mod sh;

use clap::App;
use clap::AppSettings;
use std::path::Path;
use sysinfo::{get_current_pid, ProcessExt, System, SystemExt};

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
                .arg(
                    clap::Arg::with_name("segments")
                        .required(false)
                        .min_values(0),
                )
                .about("Prints the left prompt with the specified modules"),
        )
        .subcommand(
            clap::SubCommand::with_name("rprint")
                .arg(
                    clap::Arg::with_name("segments")
                        .required(false)
                        .min_values(0),
                )
                .about("Prints the right prompt with the specified modules"),
        )
        .get_matches();

    match matches.subcommand_name().unwrap() {
        "init" => match Path::new(&shell).to_str().unwrap() {
            "bash" => print!("{}", include_str!("init.bash")),
            "zsh" => print!("{}", include_str!("init.zsh")),
            "fish" => print!("{}", include_str!("init.fish")),
            "powershell" => print!("{}", include_str!("init.powershell")),
            "ion" => print!(include_str!("init.ion")),
            _ => panic!(
                "unknown $SILVER_SHELL: \"{}\". Supported shells: bash, zsh, fish, powershell",
                shell
            ),
        },
        "lprint" => {
            print::prompt(
                &shell,
                matches
                    .subcommand_matches("lprint")
                    .unwrap()
                    .args
                    .get("segments")
                    .map(|v| &v.vals)
                    .unwrap_or(&vec![])
                    // converts OsStrs to Strings, which are Sized
                    .iter()
                    .map(|s| s.to_str().unwrap().to_owned())
                    .collect(),
                |_, (_, c, n)| {
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
                                icons::thin_left_separator(),
                            )
                        } else {
                            (
                                n.background.to_owned(),
                                c.background.to_owned(),
                                icons::left_separator(),
                            )
                        },
                    ]
                },
            );
            print!(" ")
        }
        "rprint" => print::prompt(
            &shell,
            matches
                .subcommand_matches("rprint")
                .unwrap()
                .args
                .get("segments")
                .map(|v| &v.vals)
                .unwrap_or(&vec![])
                // converts OsStrs to Strings, which are Sized
                .iter()
                .map(|s| s.to_str().unwrap().to_owned())
                .collect(),
            |_, (p, c, _)| {
                vec![
                    if p.background == c.background {
                        (
                            c.background.to_owned(),
                            c.foreground.to_owned(),
                            icons::thin_right_separator(),
                        )
                    } else {
                        (
                            p.background.to_owned(),
                            c.background.to_owned(),
                            icons::right_separator(),
                        )
                    },
                    (
                        c.background.to_owned(),
                        c.foreground.to_owned(),
                        format!(" {} ", c.value),
                    ),
                ]
            },
        ),
        _ => panic!(),
    }
}
