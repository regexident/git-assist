#[cfg(feature = "github")]
mod github;

use std::str::FromStr;

pub use self::github::*;

use async_trait::async_trait;
use git_url_parse::GitUrl;

pub const GITHUB_HOST: &str = "github.com";

pub enum SupportedHost {
    Github,
}

impl TryFrom<&GitUrl> for SupportedHost {
    type Error = anyhow::Error;

    fn try_from(url: &GitUrl) -> Result<Self, Self::Error> {
        match url.host() {
            Some(GITHUB_HOST) => {
                if cfg!(feature = "github") {
                    Ok(SupportedHost::Github)
                } else {
                    Err(anyhow::anyhow!("Github support is only available when compiled with `--features \"github\"`"))
                }
            }
            Some(host) => Err(anyhow::anyhow!("Unsupported host: {host:?}")),
            None => Err(anyhow::anyhow!("Unable to detect host: {url}")),
        }
    }
}

#[async_trait]
pub trait GitHost {
    async fn merged_pull_requests(
        &self,
        repository: &GitRepositoryUrl,
    ) -> Result<Vec<GitPullRequest>, anyhow::Error>;
}

pub struct GitPullRequest {
    pub identifier: String,
    pub title: Option<String>,

    pub base_sha: String,
    pub merge_sha: String,
}

#[derive(Clone, Debug)]
pub struct GitRepositoryUrl {
    pub url_string: String,
    pub parsed_url: GitUrl,
}

impl FromStr for GitRepositoryUrl {
    type Err = anyhow::Error;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let url_string = str.to_owned();
        let parsed_url = GitUrl::parse(str).map_err(|report| anyhow::anyhow!("{report}"))?;

        Ok(Self {
            url_string,
            parsed_url,
        })
    }
}
