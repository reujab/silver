mod cmd;
mod cmdtime;
mod conda;
mod dir;
mod git;
mod os;
mod shell;
mod status;
mod time;
mod toolbox;
mod user;
mod virtualenv;

pub use cmd::Cmd;
pub use cmdtime::Cmdtime;
pub use conda::Conda;
pub use dir::Dir;
pub use git::Git;
pub use os::OS;
pub use shell::Shell;
pub use status::{Code, Jobs};
pub use time::Time;
pub use toolbox::Toolbox;
pub use user::User;
pub use virtualenv::VirtualEnv;

pub trait Segment {
    fn draw(&self, icons: &dyn super::IconProvider) -> Option<String>;
}
