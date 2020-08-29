use crate::Segment;
use ansi_term::Style;

pub trait Print {
    fn print_right(&self, segments: &Vec<Box<dyn Segment>>) -> String;
    fn print_left(&self, segments: &Vec<Box<dyn Segment>>) -> String;
}

pub struct Separator {
    thin:  String,
    thick: String,
}

pub struct Powerline {
    right_separator: Separator,
    left_separator:  Separator,
}

impl Print for Powerline {
    fn print_left(&self, segments: &Vec<Box<dyn Segment>>) -> String {
        todo!()
    }

    fn print_right(&self, segments: &Vec<Box<dyn Segment>>) -> String {
        todo!()
    }
}
