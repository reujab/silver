#[macro_use]
extern crate lazy_static;

extern crate clap;
extern crate hostname;
extern crate humantime;
extern crate ini;
extern crate libc;
extern crate regex;
extern crate users;

mod icons;
mod modules;
mod print;
mod sh;

use clap::App;
use clap::AppSettings;
use std::env;

pub struct Segment {
    background: String,
    foreground: String,
    value: String,
}

fn main() {
    let shell = env::var("SILVER_SHELL").unwrap_or(String::new());

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
        "init" => match shell.as_str() {
            "bash" => print!("{}", include_str!("init.bash")),
            "zsh" => print!("{}", include_str!("init.zsh")),
            "fish" => print!("{}", include_str!("init.fish")),
            _ => panic!("unknown $SILVER_SHELL"),
        },
        "print" => print::prompt(
            shell,
            matches.subcommand_matches("print").unwrap().args["segments"]
                .vals
                // convert OsStrs to Strings, which are Sized
                .iter()
                .map(|s| s.to_str().unwrap().to_string())
                .collect::<Vec<String>>(),
        ),
        _ => panic!(),
    }
}
