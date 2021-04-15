use crate::{Segment, CONFIG};

use std::process::Command;

pub fn segment(segment: &mut Segment, _: &[&str]) {



	let output = Command::new(&CONFIG.usercmd.command).args(&CONFIG.usercmd.args).output().expect("failed to execute process");	

	//TODO: use String::from_utf8_lossy
	segment.value = String::from_utf8(output.stdout).expect("");

}