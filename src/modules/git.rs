use git2;
use icons;
use url::Url;
use Segment;

pub fn segment(segment: &mut Segment, _: &[&str]) {
    match git2::Repository::discover(".") {
        Ok(mut repo) => {
            let domain = match repo.find_remote("origin") {
                Ok(origin) => match Url::parse(origin.url().unwrap_or(""))
                    .unwrap_or(Url::parse("https://example.com").unwrap())
                    .domain()
                    .unwrap_or("")
                {
                    "github.com" => icons::get("github"),
                    "gitlab.com" => icons::get("gitlab"),
                    "bitbucket.com" => icons::get("bitbucket"),
                    _ => icons::get("git"),
                },
                Err(_) => icons::get("git"),
            }.to_owned();

            let mut stashes = String::new();
            repo.stash_foreach(|_, _, _| {
                stashes += &icons::get("stash");
                true
            }).unwrap();

            let mut ahead = 0;
            let mut behind = 0;
            // let branch;
            match repo.head() {
                Ok(head) => {
                    let local = head.target().unwrap();
                    let upstream =
                        repo.find_branch(head.shorthand().unwrap(), git2::BranchType::Local)
                            .unwrap();
                    let upstream = upstream.upstream().unwrap().get().target().unwrap();
                    let (_ahead, _behind) = repo.graph_ahead_behind(local, upstream).unwrap();
                    ahead = _ahead;
                    behind = _behind;
                }
                Err(_) => {}
            }
            println!("{} - {}", ahead, behind);

            segment.value = [domain, stashes].join(" ");
        }
        Err(_) => {}
    }
}
