use crossterm::style::StyledContent;

pub enum Shell {
    Zsh,
    Fish,
    Bash,
    Powershell,
    Ion,
}

impl Shell {
    fn escape_seq(&self, seq: &str) -> String {
        match self {
            Self::Zsh => format!("%{{{}%}}", seq),
            Self::Bash => format!(r"\[{}\]", seq),
            _ => seq.to_owned(),
        }
    }

    fn style(&self, text: &StyledContent) {
        let style = text.style();
    }
}
