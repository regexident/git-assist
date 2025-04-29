use std::{path::PathBuf, process::ExitStatus, str::FromStr};

use clap::{Parser, Subcommand};
use git2::{Remote as GitRemote, Repository as GitRepository};
use git_assist::{
    command::bisect::{skip_pull_requests, SkipPullRequestsConfig},
    host::{GitHost, GitRepositoryUrl, GithubApi, SupportedHost},
};
use inquire::{Select, Text};

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

enum RepositoryUrlChoice<'a> {
    Remote(&'a GitRemote<'a>),
    Custom,
}

impl std::fmt::Display for RepositoryUrlChoice<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepositoryUrlChoice::Remote(remote) => {
                let url = remote.url().unwrap_or_default();

                write!(f, "{url}")
            }
            RepositoryUrlChoice::Custom => {
                write!(f, "Custom url ...")
            }
        }
    }
}

impl SkipPullRequestsCommand {
    pub async fn run(&self) -> anyhow::Result<ExitStatus> {
        let config = self.as_config()?;

        let host: Box<dyn GitHost> = match SupportedHost::try_from(&config.repository.parsed_url)? {
            SupportedHost::Github => Box::new(GithubApi::authenticated()?),
        };

        skip_pull_requests(&*host, &config).await
    }

    fn as_config(&self) -> anyhow::Result<SkipPullRequestsConfig> {
        let directory = match &self.directory {
            Some(directory) => PathBuf::from(shellexpand::tilde(directory).as_ref()),
            None => std::env::current_dir()?,
        };

        let repository = GitRepository::open(&directory)?;

        let remotes: Vec<GitRemote<'_>> = repository
            .remotes()?
            .into_iter()
            .filter_map(|name| name.and_then(|name| repository.find_remote(name).ok()))
            .collect();

        let url = match &self.remote_url {
            Some(repository_url) => repository_url.to_owned(),
            None => {
                let mut choices: Vec<_> = remotes.iter().map(RepositoryUrlChoice::Remote).collect();
                choices.push(RepositoryUrlChoice::Custom);

                let choice = Select::new("Remote url:", choices).prompt()?;

                match choice {
                    RepositoryUrlChoice::Remote(remote) => remote.url().unwrap().to_owned(),
                    RepositoryUrlChoice::Custom => Text::new("Remote url:").prompt()?,
                }
            }
        };

        let repository = GitRepositoryUrl::from_str(&url)?;

        let good: String = match &self.good {
            Some(good) => good.to_owned(),
            None => Text::new("Known good commit:").prompt()?,
        }
        .trim()
        .to_owned();

        let bad: String = match &self.bad {
            Some(bad) => bad.to_owned(),
            None => Text::new("Known bad commit:").prompt()?,
        }
        .trim()
        .to_owned();

        let dry_run = self.dry_run;

        Ok(SkipPullRequestsConfig {
            repository,
            directory,
            good,
            bad,
            dry_run,
        })
    }
}
