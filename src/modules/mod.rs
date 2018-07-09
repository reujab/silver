use Segment;

mod cmdtime;
mod dir;
mod os;
mod status;
mod time;
mod user;
mod env;
mod virtualenv;

pub fn handle(module: &str, segment: &mut Segment, args: &[&str]) {
    match module {
        "os" => os::segment(segment, args),
        "status" => status::segment(segment, args),
        "dir" => dir::segment(segment, args),
        "user" => user::segment(segment, args),
        "cmdtime" => cmdtime::segment(segment, args),
        "time" => time::segment(segment, args),
        "env" => env::segment(segment, args),
        "virtualenv" => virtualenv::segment(segment, args),
        _ => panic!("unknown module, {}", module),
    }
}
