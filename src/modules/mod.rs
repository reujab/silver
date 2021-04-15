mod cmdtime;
mod conda;
mod dir;
mod env;
mod git;
mod os;
mod shell;
mod status;
mod time;
mod toolbox;
mod user;
mod virtualenv;
mod usercmd;

use crate::Segment;

pub fn handle(module: &str, segment: &mut Segment, args: &[&str]) {
    match module {
        "os" => os::segment(segment, args),
        "status" => status::segment(segment, args),
        "dir" => dir::segment(segment, args),
        "git" => git::segment(segment, args),
        "user" => user::segment(segment, args),
        "cmdtime" => cmdtime::segment(segment, args),
        "time" => time::segment(segment, args),
        "virtualenv" => virtualenv::segment(segment, args),
        "conda" => conda::segment(segment, args),
        "toolbox" => toolbox::segment(segment, args),
        "shell" => shell::segment(segment, args),
		"usercmd" => usercmd::segment(segment, args),
        "env" => env::segment(segment, args),
        _ => panic!("unknown module, {}", module),
    }
}
