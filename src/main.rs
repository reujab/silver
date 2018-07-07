extern crate clap;

use clap::App;
use std::env;

fn main() {
    let shell = env::var("BRONZE_SHELL").expect("$BRONZE_SHELL is not set");

    let matches = App::new("bronze")
        .setting(clap::AppSettings::SubcommandRequired)
        .subcommand(clap::SubCommand::with_name("init"))
        .get_matches();

    match matches.subcommand().0 {
        "init" => match shell.as_str() {
            "bash" => print!("{}", include_str!("init.bash")),
            "zsh" => print!("{}", include_str!("init.zsh")),
            "fish" => print!("{}", include_str!("init.fish")),
            _ => panic!("unknown $BRONZE_SHELL"),
        },
        _ => panic!(),
    }
}
