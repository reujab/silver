extern crate clap;

mod print;

use clap::App;
use clap::AppSettings;
use std::env;

fn main() {
    let shell = env::var("BRONZE_SHELL").expect("$BRONZE_SHELL is not set");

    let matches = App::new("bronze")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(clap::SubCommand::with_name("init"))
        .subcommand(
            clap::SubCommand::with_name("print").arg(
                clap::Arg::with_name("segments")
                    .required(true)
                    .min_values(1),
            ),
        )
        .get_matches();

    match matches.subcommand_name().unwrap() {
        "init" => match shell.as_str() {
            "bash" => print!("{}", include_str!("init.bash")),
            "zsh" => print!("{}", include_str!("init.zsh")),
            "fish" => print!("{}", include_str!("init.fish")),
            _ => panic!("unknown $BRONZE_SHELL"),
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
