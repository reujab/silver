use crate::{Segment, CONFIG};

use std::process::Command;

pub fn segment(segment: &mut Segment, _: &[&str]) {
	let output = Command::new(&CONFIG.usercmd.command)
		.args(&CONFIG.usercmd.args)
		.output()
		.expect("failed to execute process");
	segment.value = String::from((String::from_utf8(output.stdout).expect("")).trim());
}