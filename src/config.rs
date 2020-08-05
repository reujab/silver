use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

type Prompt = Vec<Segment>;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub separator: Separators,
    #[serde(default)]
    pub left: Prompt,
    #[serde(default)]
    pub right: Prompt,
    #[serde(default)]
    pub icons: HashMap<String, String>,
    #[serde(default)]
    pub icon_set: IconSet,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Separators {
    pub right: Separator,
    pub left: Separator,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Separator {
    pub thick: String,
    pub thin: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Segment {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub color: Colors,
    #[serde(default)]
    pub args: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Colors {
    #[serde(default)]
    pub background: Color,
    pub foreground: Color,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum Color {
    Hex(u32),
    Name(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IconSet {
    Nerd,
    Unicode,
    ASCII,
}

impl Default for Separators {
    fn default() -> Self {
        Self {
            left: Separator {
                thick: "\u{e0b0}".into(),
                thin: "\u{e0b1}".into(),
            },
            right: Separator {
                thick: "\u{e0b2}".into(),
                thin: "\u{e0b3}".into(),
            },
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::Name("none".into())
    }
}

impl Default for IconSet {
    fn default() -> Self {
        Self::Nerd
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Name(name) => write!(f, "{}", name),
            Self::Hex(num) => write!(f, "{:x}", num),
        }
    }
}
