use crate::{icons, Segment, CONFIG};

use std::{env, path::Path};

pub fn segment(segment: &mut Segment, _: &[&str]) {
    let mut wd = match env::current_dir() {
        Ok(wd) => wd,
        Err(_) => return,
    };

    // processes aliases
    let mut aliases = CONFIG
        .dir
        .aliases
        .iter()
        .flat_map(|(i, j)| {
            if let Ok(p) = Path::new(
                &shellexpand::full_with_context_no_errors(
                    j,
                    || dirs::home_dir().map(|p| p.to_string_lossy().into_owned()),
                    |s| env::var(s).map(Some).unwrap_or_default(),
                )
                .into_owned(),
            )
            .canonicalize()
            {
                Some((p, Path::new(i).to_owned()))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    // default home alias
    if let Some(home) = dirs::home_dir() {
        aliases.push((home, Path::new(&icons::get("home")).to_owned()))
    }
    // sorts from deepest alias to shallowest
    aliases.sort_by(|a, b| {
        a.0.iter()
            .count()
            .partial_cmp(&b.0.iter().count())
            .unwrap()
            .reverse()
    });
    for (dir, alias) in aliases {
        wd = match wd.strip_prefix(dir) {
            Ok(stripped) => alias.join(stripped),
            Err(_) => wd.clone(),
        }
    }

    // processes length
    if let Some(len) = CONFIG.dir.length {
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

    segment.value = wd.to_str().unwrap().to_owned();

    if segment.value != "/" && segment.value.ends_with('/') {
        segment.value.pop();
    }
}
