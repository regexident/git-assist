use std::{
    collections::HashSet,
    os::unix::process::ExitStatusExt,
    path::PathBuf,
    process::{Command, ExitStatus},
};

use git2::{Oid, Repository as GitRepository};

use crate::{
    git::commits_in_range,
    host::{GitHost, GitPullRequest, GitRepositoryUrl},
};

pub struct SkipPullRequestsConfig {
    /// The git repository.
    pub repository: GitRepositoryUrl,

    // The git directory.
    pub directory: PathBuf,

    /// A known "good" git commit.
    pub good: String,

    /// A known "bad" git commit.
    pub bad: String,

    /// Perform a "dry" run.
    pub dry_run: bool,
}

pub async fn skip_pull_requests(
    host: &dyn GitHost,
    config: &SkipPullRequestsConfig,
) -> anyhow::Result<ExitStatus> {
    eprintln!("Opening git repository ...");
    let repository = GitRepository::open(&config.directory)?;

    let range_commit_ids: HashSet<Oid> = {
        let range = (
            repository.revparse_single(&config.good)?.id(),
            repository.revparse_single(&config.bad)?.id(),
        );
        commits_in_range(&repository, range)?
            .into_iter()
            .map(|commit| commit.id())
            .collect()
    };

    eprintln!("Requesting pull requests ...");
    let pull_requests = host.merged_pull_requests(&config.repository).await?;

    eprintln!("Filtering pull requests ...");
    let pull_requests: Vec<GitPullRequest> = pull_requests
        .into_iter()
        .filter(|pull_request| {
            let Ok(base_obj) = repository.revparse_single(&pull_request.base_sha) else {
                return false;
            };
            let Ok(merge_obj) = repository.revparse_single(&pull_request.merge_sha) else {
                return false;
            };

            range_commit_ids.contains(&base_obj.id()) || range_commit_ids.contains(&merge_obj.id())
        })
        .collect();

    if !config.dry_run && std::env::current_dir()? != config.directory {
        eprintln!("Entering repository directory ...");
        std::env::set_current_dir(&config.directory)?;
    }

    for pull_request in pull_requests {
        let program = "git";
        let mut command = Command::new(program);
        let mut args = vec!["bisect".to_owned(), "skip".to_owned()];

        args.push(format!(
            "{start}..{end}^",
            start = pull_request.base_sha,
            end = pull_request.merge_sha,
        ));

        let formatted_args = args.join(" ");

        command.args(args);

        println!(
            "# Pull request #{number}: {title:?}",
            number = pull_request.identifier,
            title = pull_request.title
        );

        if config.dry_run {
            println!("{program} {formatted_args}");
        } else {
            command.spawn().and_then(|mut child| child.wait())?;
        }
    }

    Ok(ExitStatus::from_raw(0))
}
