pub use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    about = "a cross-shell customizable powerline-like prompt with icons",
    name = "silver",
    after_help = "https://github.com/reujab/silver/wiki"
)]
pub struct Silver {
    #[structopt(short, long)]
    pub config: Option<String>,
    #[structopt(subcommand)]
    pub cmd:    Command,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    Init,
    Lprint,
    Rprint,
}
