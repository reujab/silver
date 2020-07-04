use modules;
use sh;
use std::iter::once;
use Segment;

pub fn prompt<T, U>(shell: &str, args: Vec<String>, f: T)
where
    T: Fn(usize, (&Segment, &Segment, &Segment)) -> U,
    U: IntoIterator<Item = (String, String, String)>,
{
    let v: Vec<_> = once(Segment::default())
        .chain(
            args.into_iter()
                .map(|arg| {
                    let fields: Vec<_> = arg.split(':').collect();
                    if fields.len() < 3 {
                        panic!("invalid argument, {}", arg);
                    }

                    let mut segment = Segment {
                        background: fields[1].to_owned(),
                        foreground: fields[2].to_owned(),
                        ..Default::default()
                    };
                    modules::handle(fields[0], &mut segment, &fields[3..]);
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
