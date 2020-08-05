use crate::icons;
use crate::Segment;
use crate::CONFIG;

use std::env;
use std::path::Path;

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
        .map(|(i, j)| (j.to_owned(), i.to_owned()))
        .collect::<Vec<_>>();
    // default home alias
    if let Some(home) = dirs::home_dir() {
        aliases.push((home.to_str().unwrap().to_owned(), icons::get("home")))
    }
    // sorts from deepest alias to shallowest
    aliases.sort_by(|a, b| {
        Path::new(&a.0)
            .iter()
            .count()
            .partial_cmp(&Path::new(&b.0).iter().count())
            .unwrap()
            .reverse()
    });
    for (dir, alias) in aliases {
        wd = match wd.strip_prefix(dir) {
            Ok(stripped) => Path::new(&alias).join(stripped.to_path_buf()),
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
