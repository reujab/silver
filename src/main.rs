#[macro_use]
extern crate lazy_static;

extern crate chrono;
extern crate clap;
extern crate dirs;
extern crate git2;
extern crate hostname;
extern crate humantime;
extern crate ini;
extern crate libc;
extern crate regex;
extern crate url;
#[cfg(not(target_os = "windows"))]
extern crate users;
#[cfg(target_os = "windows")]
extern crate winapi;

mod icons;
mod modules;
mod print;
mod sh;

use clap::App;
use clap::AppSettings;
use std::env;
use std::path::Path;

pub struct Segment {
    background: String,
    foreground: String,
    value: String,
}

fn main() {
    let shell = env::var("SILVER_SHELL").unwrap_or_default();

    let matches = App::new("silver")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .about("a cross-shell customizable powerline-like prompt with icons")
        .after_help("https://github.com/reujab/silver/wiki")
        .subcommand(
            clap::SubCommand::with_name("init").about("Initializes the shell for use of silver"),
        )
        .subcommand(
            clap::SubCommand::with_name("print")
                .arg(
                    clap::Arg::with_name("segments")
                        .required(true)
                        .min_values(1),
                )
                .about("Prints the prompt with the specified modules"),
        )
        .get_matches();

    match matches.subcommand_name().unwrap() {
        "init" => match Path::new(&shell).to_str().unwrap() {
            "bash" => print!("{}", include_str!("init.bash")),
            "zsh" => print!("{}", include_str!("init.zsh")),
            "fish" => print!("{}", include_str!("init.fish")),
            "powershell" => print!("{}", include_str!("init.powershell")),
            _ => panic!(
                "unknown $SILVER_SHELL: \"{}\". Supported shells: bash, zsh, fish, powershell",
                shell
            ),
        },
        "print" => print::prompt(
            &shell,
            matches.subcommand_matches("print").unwrap().args["segments"]
                .vals
                // converts OsStrs to Strings, which are Sized
                .iter()
                .map(|s| s.to_str().unwrap().to_owned())
                .collect::<Vec<String>>(),
        ),
        _ => panic!(),
    }
}
