pub use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    about = "a cross-shell customizable powerline-like prompt with icons",
    name = "silver",
    after_help = r#"To use silver, put `eval $(silver init)` or equivalent in your shell's
interactive startup file.  The `lprint` and `rprint` subcommands are
used by the code emitted by `silver init`; you should not need to use
them directly.

silver expects a configuration file in $XDG_CONFIG_HOME/silver/config.toml
on Unix, or {FOLDERID_RoamingAppData}/silver/config/config.toml on Windows.
(This path can be overridden with the --config option.)
See <https://github.com/reujab/silver/wiki> for documentation of this file.
"#
)]
pub struct Silver {
    /// Path of the configuration file to use.
    #[structopt(short, long)]
    pub config: Option<String>,
    /// Name of your shell (e.g. bash).
    #[structopt(short, long)]
    pub shell:  Option<String>,
    #[structopt(subcommand)]
    pub cmd:    Command,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    /// Emit shell code to set up silver.
    Init,
    /// Print the left side of a prompt.
    Lprint,
    /// Print the right side of a prompt.
    Rprint,
}
