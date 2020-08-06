use crate::config;
use crate::modules;
use crate::sh;
use crate::Segment;
use std::iter::once;

pub fn prompt<T, U>(shell: &str, args: &Vec<config::Segment>, f: T)
where
    T: Fn(usize, (&Segment, &Segment, &Segment)) -> U,
    U: IntoIterator<Item = (String, String, String)>,
{
    let v: Vec<_> = once(Segment::default())
        .chain(
            args.into_iter()
                .map(|arg| {
                    let mut segment = Segment {
                        background: arg.color.background.to_string(),
                        foreground: arg.color.foreground.to_string(),
                        ..Default::default()
                    };
                    modules::handle(
                        arg.name.as_str(),
                        &mut segment,
                        arg.args
                            .iter()
                            .map(String::as_str)
                            .collect::<Vec<_>>()
                            .as_slice(),
                    );
                    segment
                })
                .filter(|seg| !seg.value.is_empty()),
        )
        .chain(once(Segment::default()))
        .collect();

    let n = v.len() - 2;
    (0..n)
        .flat_map(|i| f(i, (&v[i], &v[i + 1], &v[i + 2])))
        .for_each(|(bg, fg, val)| {
            print!(
                "{}{}{}",
                sh::escape_background(shell, &bg),
                sh::escape_foreground(shell, &fg),
                val,
            );
        });
    sh::reset_colors(shell);
}
