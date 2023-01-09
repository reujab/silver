pub use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(
    about = "a cross-shell customizable powerline-like prompt with icons",
    name = "silver",
    after_help = "https://github.com/reujab/silver/wiki"
)]
pub struct Silver {
    #[arg(short, long)]
    pub config: Option<String>,

    #[command(subcommand)]
    pub cmd:    Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Init,
    Lprint,
    Rprint,
}
