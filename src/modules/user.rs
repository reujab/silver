use hostname;
use users;
use Segment;

pub fn segment(segment: &mut Segment, _: &[&str]) {
    segment.value = format!(
        "{}@{}",
        users::get_current_username()
            .unwrap()
            .into_string()
            .unwrap(),
        hostname::get_hostname().unwrap()
    )
}
