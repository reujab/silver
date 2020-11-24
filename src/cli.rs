pub use clap::Clap;

#[derive(Clap, Debug)]
#[clap(
    about = "a cross-shell customizable powerline-like prompt with icons",
    name = "silver",
    after_help = "https://github.com/reujab/silver/wiki"
)]
pub struct Silver {
    #[clap(short, long)]
    pub config: Option<String>,
    #[clap(subcommand)]
    pub cmd: Command,
}

#[derive(Clap, Debug)]
pub enum Command {
    Init,
    Lprint,
    Rprint,
}
