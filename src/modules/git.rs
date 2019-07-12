use git2;
use icons;
use url::Url;
use Segment;

pub fn segment(segment: &mut Segment, _: &[&str]) {
    if let Ok(mut repo) = git2::Repository::discover(".") {
        let mut domain = icons::get("git");
        if let Ok(origin) = repo.find_remote("origin") {
            if let Ok(url) = Url::parse(origin.url().unwrap_or_default()) {
                match url.domain().unwrap_or_default() {
                    "github.com" => domain = icons::get("github"),
                    "gitlab.com" => domain = icons::get("gitlab"),
                    "bitbucket.com" => domain = icons::get("bitbucket"),
                    "dev.azure.com" => domain = icons::get("azure"),
                    _ => {}
                }
            }
        }

        let mut stashes = 0;
        repo.stash_foreach(|_, _, _| {
            stashes += 1;
            true
        })
        .unwrap();
        let stashes = icons::repeat("stash", stashes);

        let mut graph = String::new();
        let mut branch = String::new();
        if let Ok(head) = repo.head() {
            branch = head.shorthand().unwrap_or_default().to_owned();
            let local = head.target().unwrap();
            if let Ok(local_branch) = repo.find_branch(&branch, git2::BranchType::Local) {
                let upstream = local_branch.upstream();
                if let Ok(upstream) = upstream {
                    if let Some(upstream) = upstream.get().target() {
                        let (ahead, behind) = repo.graph_ahead_behind(local, upstream).unwrap();
                        graph = icons::repeat("ahead", ahead) + &icons::repeat("behind", behind);
                    }
                }
            }
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
