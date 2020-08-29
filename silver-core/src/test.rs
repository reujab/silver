use crate::Shell;
use ansi_term::{Color, Style};

#[test]
fn escape() {
    assert_eq!(
        &Shell::Zsh.escape(
            &Style::new()
                .bold()
                .fg(Color::RGB(0xc0, 0xff, 0xee))
                .on(Color::Black)
                .italic()
        ),
        "%{\x1b[1;3;40;38;2;192;255;238m%}"
    );
}
