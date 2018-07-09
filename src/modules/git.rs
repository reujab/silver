use git2;
use icons;
use url::Url;
use Segment;

pub fn segment(segment: &mut Segment, _: &[&str]) {
    if let Ok(mut repo) = git2::Repository::discover(".") {
        let domain = match repo.find_remote("origin") {
            Ok(origin) => match Url::parse(origin.url().unwrap_or(""))
                .unwrap_or_else(|_| Url::parse("https://example.com").unwrap())
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

        let mut graph = String::new();
        let mut branch = String::new();
        if let Ok(head) = repo.head() {
            branch = head.shorthand().unwrap().to_owned();

            let local = head.target().unwrap();
            let upstream = repo.find_branch(&branch, git2::BranchType::Local).unwrap();
            let upstream = upstream.upstream().unwrap().get().target().unwrap();

            let (ahead, behind) = repo.graph_ahead_behind(local, upstream).unwrap();
            graph = icons::get("ahead").repeat(ahead) + &icons::get("behind").repeat(behind);
        }

        let mut modified = String::new();
        let mut staged = String::new();
        if let Ok(statuses) =
            repo.statuses(Some(git2::StatusOptions::new().include_untracked(true)))
        {
            for status in statuses.iter() {
                // dirty
                segment.background = "yellow".to_owned();

                let status = status.status();
                if modified.is_empty() && status.is_wt_new()
                    || status.is_wt_modified()
                    || status.is_wt_renamed()
                    || status.is_wt_typechange()
                {
                    modified = icons::get("modified");
                }
                if staged.is_empty() && status.is_index_new()
                    || status.is_index_modified()
                    || status.is_index_deleted()
                    || status.is_index_renamed()
                    || status.is_index_typechange()
                {
                    staged = icons::get("staged");
                }

                if !modified.is_empty() && !staged.is_empty() {
                    break;
                }
            }
        }

        let mut parts = vec![domain];
        if !stashes.is_empty() || !graph.is_empty() {
            parts.push(stashes + &graph);
        }
        if !branch.is_empty() {
            parts.push(branch);
        }
        if !modified.is_empty() || !staged.is_empty() {
            parts.push(modified + &staged);
        }
        segment.value = parts.join(" ");
    }
}
