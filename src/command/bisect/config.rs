use std::{path::PathBuf, str::FromStr};

use git2::{Remote as GitRemote, Repository as GitRepository};
use inquire::{Select, Text};

use crate::host::GitRepositoryUrl;

use super::SkipPullRequestsConfig;

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

/// Builder for creating `SkipPullRequestsConfig` from command-line arguments and user input.
pub struct SkipPullRequestsConfigBuilder {
    pub remote_url: Option<String>,
    pub directory: Option<String>,
    pub good: Option<String>,
    pub bad: Option<String>,
    pub dry_run: bool,
}

impl Default for SkipPullRequestsConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SkipPullRequestsConfigBuilder {
    pub fn new() -> Self {
        Self {
            remote_url: None,
            directory: None,
            good: None,
            bad: None,
            dry_run: false,
        }
    }

    pub fn remote_url(mut self, remote_url: Option<String>) -> Self {
        self.remote_url = remote_url;
        self
    }

    pub fn directory(mut self, directory: Option<String>) -> Self {
        self.directory = directory;
        self
    }

    pub fn good(mut self, good: Option<String>) -> Self {
        self.good = good;
        self
    }

    pub fn bad(mut self, bad: Option<String>) -> Self {
        self.bad = bad;
        self
    }

    pub fn dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = dry_run;
        self
    }

    pub fn build(self) -> anyhow::Result<SkipPullRequestsConfig> {
        let directory = match self.directory {
            Some(directory) => PathBuf::from(shellexpand::tilde(&directory).as_ref()),
            None => std::env::current_dir()?,
        };

        let repository_handle = GitRepository::open(&directory)?;

        let remotes: Vec<GitRemote<'_>> = repository_handle
            .remotes()?
            .into_iter()
            .filter_map(|name| {
                name.and_then(|name| match repository_handle.find_remote(name) {
                    Ok(remote) => Some(remote),
                    Err(err) => {
                        eprintln!("Warning: Failed to find remote '{name}': {err}");
                        None
                    }
                })
            })
            .collect();

        let url = match self.remote_url {
            Some(repository_url) => repository_url,
            None => {
                let mut choices: Vec<_> = remotes.iter().map(RepositoryUrlChoice::Remote).collect();
                choices.push(RepositoryUrlChoice::Custom);

                let choice = Select::new("Remote url:", choices).prompt()?;

                match choice {
                    RepositoryUrlChoice::Remote(remote) => remote
                        .url()
                        .ok_or_else(|| anyhow::anyhow!("Remote has no URL configured"))?
                        .to_owned(),
                    RepositoryUrlChoice::Custom => Text::new("Remote url:").prompt()?,
                }
            }
        };

        let repository = GitRepositoryUrl::from_str(&url)?;

        let good: String = match self.good {
            Some(good) => good,
            None => Text::new("Known good commit:").prompt()?,
        }
        .trim()
        .to_owned();

        let bad: String = match self.bad {
            Some(bad) => bad,
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
