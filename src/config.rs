use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt, time::Duration};

type Prompt = Vec<Segment>;

#[derive(Debug, Serialize, Deserialize)]
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
    #[serde(default)]
    #[serde(with = "humantime_serde")]
    pub cmdtime_threshold: Duration,
    #[serde(default)]
    pub dir: Dir,
    #[serde(default)]
    pub git: Git,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Separators {
    pub right: Separator,
    pub left:  Separator,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Separator {
    pub thick: String,
    pub thin:  String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Segment {
    #[serde(default)]
    pub name:  String,
    #[serde(default)]
    pub color: Colors,
    #[serde(default)]
    pub args:  Vec<String>,
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
    #[allow(clippy::upper_case_acronyms)]
    ASCII,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Dir {
    #[serde(default)]
    pub aliases: HashMap<String, String>,
    #[serde(default)]
    pub length:  Option<usize>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Git {
    #[serde(default)]
    pub ignore_dirs: Vec<String>,
}

fn default_left_prompt() -> Prompt {
    vec![Segment {
        name:  "dir".into(),
        color: Colors {
            background: Color::Name("7".into()),
            foreground: Color::Name("0".into()),
        },
        args:  vec![],
    }]
}

// Implement defaults that will render a
// usable prompt.
impl std::default::Default for Config {
    fn default() -> Self {
        let config_dir = confy::get_configuration_file_path("silver", None)
            .expect("Failed to get configuration path")
            .into_os_string()
            .into_string();

        match config_dir {
            Ok(config_dir) => eprintln!("Creating default configuration at: {}", config_dir),
            Err(_) => eprintln!("Couldn't format the path to the configuration file.  Oops"),
        }

        Self {
            left: default_left_prompt(),
            separator: Separators::default(),
            right: Prompt::default(),
            icons: HashMap::default(),
            icon_set: IconSet::default(),
            cmdtime_threshold: Duration::default(),
            dir: Dir::default(),
            git: Git::default(),
        }
    }
}

impl Default for Separators {
    fn default() -> Self {
        Self {
            left:  Separator {
                thick: "\u{e0b0}".into(),
                thin:  "\u{e0b1}".into(),
            },
            right: Separator {
                thick: "\u{e0b2}".into(),
                thin:  "\u{e0b3}".into(),
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
        Self::Unicode
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
