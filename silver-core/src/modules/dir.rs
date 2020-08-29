use super::Segment;

use std::{
    collections::HashMap,
    env,
    path::{Path, PathBuf},
};

pub struct Dir {
    pub aliases:    HashMap<String, PathBuf>,
    pub max_length: Option<usize>,
}

impl Segment for Dir {
    fn draw(&self, icons: &dyn crate::IconProvider) -> Option<String> {
        let mut wd = env::current_dir().ok()?;

        let readonly = wd.metadata().ok()?.permissions().readonly();

        // processes aliases
        let mut aliases = self
            .aliases
            .iter()
            .flat_map(|(i, j)| {
                if let Ok(p) = PathBuf::from(
                    shellexpand::full_with_context_no_errors(
                        j.to_str().unwrap_or_default(),
                        dirs::home_dir,
                        |s| env::var(s).map(Some).unwrap_or_default(),
                    )
                    .into_owned(),
                )
                .canonicalize()
                {
                    Some((p, i.to_owned()))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        if let Some(home) = dirs::home_dir() {
            if let Some(home_icon) = icons.get("home").map(|c| c.to_string()) {
                aliases.push((home, home_icon))
            }
        }

        // sorts from deepest alias to shallowest
        aliases.sort_unstable_by_key(|i| i.0.iter().count());
        aliases.reverse();
        for (dir, alias) in aliases {
            if let Ok(stripped) = wd.strip_prefix(dir) {
                wd = Path::new(&alias).join(stripped.to_path_buf())
            }
        }

        // processes length
        if let Some(len) = self.max_length {
            let iter_len = wd.iter().count();
            let mut i = 0;
            wd = wd
                .iter()
                .map(|component| {
                    i += 1;
                    if i != iter_len && component.len() > len {
                        component.to_str().unwrap().chars().take(len).collect()
                    } else {
                        component.to_str().unwrap().to_owned()
                    }
                })
                .collect();
        }

        let mut ret = wd.to_str()?.to_owned();

        if ret != "/" && ret.ends_with('/') {
            ret = ret.strip_suffix('/')?.to_owned()
        }

        if readonly {
            ret = format!(
                "{icon}{dir}",
                dir = ret,
                icon = icons.get("readonly").unwrap_or_default()
            )
        }

        Some(ret)
    }
}
