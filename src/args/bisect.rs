use std::process::ExitStatus;

use clap::{Parser, Subcommand};
use git_assist::{
    command::bisect::{skip_pull_requests, SkipPullRequestsConfigBuilder},
    host::{GitHost, GithubApi, SupportedHost},
};

use super::CommonOptions;

#[derive(Subcommand, Eq, PartialEq, Debug)]
pub(crate) enum Command {
    /// A sub-command for skipping all internal pull request commits (i.e. `base..head^`).
    SkipPullRequests(SkipPullRequestsCommand),
}

#[derive(Parser, Eq, PartialEq, Debug)]
pub(crate) struct SkipPullRequestsCommand {
    /// Remote url to fetch pull requests from.
    #[arg(long)]
    pub(crate) remote_url: Option<String>,

    /// Directory of the repository.
    #[arg(long, hide = true)]
    pub(crate) directory: Option<String>,

    /// A known "good" commit.
    #[arg(long)]
    pub(crate) good: Option<String>,

    /// A known "bad" commit.
    #[arg(long)]
    pub(crate) bad: Option<String>,

    /// Perform a "dry" run.
    #[arg(long)]
    pub(crate) dry_run: bool,

    /// Common options.
    #[command(flatten)]
    pub(crate) common: CommonOptions,
}

impl SkipPullRequestsCommand {
    pub async fn run(&self) -> anyhow::Result<ExitStatus> {
        let config = SkipPullRequestsConfigBuilder::new()
            .remote_url(self.remote_url.clone())
            .directory(self.directory.clone())
            .good(self.good.clone())
            .bad(self.bad.clone())
            .dry_run(self.dry_run)
            .build()?;

        let host: Box<dyn GitHost> = match SupportedHost::try_from(&config.repository.parsed_url)? {
            SupportedHost::Github => Box::new(GithubApi::authenticated()?),
        };

        skip_pull_requests(&*host, &config).await
    }
}
