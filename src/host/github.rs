use std::str::FromStr;

use async_trait::async_trait;
use git_url_parse::types::provider::GenericProvider;
use inquire::{Password, PasswordDisplayMode, Select, Text};
use jsonwebtoken::EncodingKey;
use octocrab::{
    auth::{AppAuth, Auth as GithubAuthentication},
    models::AppId,
    params::{pulls::Sort, Direction, State},
    Octocrab, OctocrabBuilder,
};
use secrecy::{ExposeSecret, SecretString};

use crate::host::{GitHost, GitPullRequest, GitRepositoryUrl, GITHUB_HOST};

#[derive(Clone, Default, Debug)]
pub struct GithubApi {
    api: Octocrab,
}

impl GithubApi {
    pub fn authenticated() -> anyhow::Result<Self> {
        let mut builder = OctocrabBuilder::default();

        builder = match pick_authentication()? {
            GithubAuthentication::None => builder,
            GithubAuthentication::Basic { username, password } => {
                builder.basic_auth(username, password)
            }
            GithubAuthentication::PersonalToken(token) => {
                builder.personal_token(token.expose_secret().to_owned())
            }
            GithubAuthentication::App(AppAuth { app_id, key }) => builder.app(app_id, key),
            GithubAuthentication::OAuth(oauth) => builder.oauth(oauth),
            GithubAuthentication::UserAccessToken(token) => {
                builder.user_access_token(token.expose_secret().to_owned())
            }
        };

        let api = builder.build()?;

        Ok(Self { api })
    }
}

#[derive(Clone, Debug)]
pub struct GithubRepository(GitRepositoryUrl);

impl TryFrom<GitRepositoryUrl> for GithubRepository {
    type Error = anyhow::Error;

    fn try_from(repository: GitRepositoryUrl) -> Result<Self, anyhow::Error> {
        let url = &repository.url_string;
        match repository.parsed_url.host() {
            Some(GITHUB_HOST) => {}
            Some(_) => {
                anyhow::bail!("Not a Github url: {url}");
            }
            None => {
                anyhow::bail!("No host found in url: {url}");
            }
        }

        Ok(Self(repository))
    }
}

impl GithubRepository {
    pub fn owner(&self) -> Result<String, anyhow::Error> {
        let provider: GenericProvider = self.0.parsed_url.provider_info()?;
        Ok(provider.owner().clone())
    }

    pub fn name(&self) -> Result<String, anyhow::Error> {
        let provider: GenericProvider = self.0.parsed_url.provider_info()?;
        Ok(provider.repo().clone())
    }
}

#[async_trait]
impl GitHost for GithubApi {
    async fn merged_pull_requests(
        &self,
        repository: &GitRepositoryUrl,
    ) -> Result<Vec<GitPullRequest>, anyhow::Error> {
        let safe_repository = GithubRepository::try_from(repository.clone())?;

        let pull_requests = self
            .api
            .all_pages(
                self.api
                    .pulls(safe_repository.owner()?, safe_repository.name()?)
                    .list()
                    .state(State::Closed)
                    .sort(Sort::Created)
                    .direction(Direction::Ascending)
                    .per_page(100)
                    .send()
                    .await?,
            )
            .await?;

        let pull_requests: Vec<GitPullRequest> = pull_requests
            .into_iter()
            .filter(|pull_request| pull_request.merged_at.is_some())
            .map(|pull_request| {
                let identifier = pull_request.number.to_string();
                let title = pull_request.title;
                let base_sha = pull_request.base.sha;
                let Some(merge_sha) = pull_request.merge_commit_sha else {
                    anyhow::bail!("Could not find merge commit sha");
                };

                Ok(GitPullRequest {
                    identifier,
                    title,
                    base_sha,
                    merge_sha,
                })
            })
            .collect::<Result<_, _>>()?;

        Ok(pull_requests)
    }
}

fn pick_authentication() -> anyhow::Result<GithubAuthentication> {
    enum AuthKind {
        None,
        Basic,
        PersonalToken,
        App,
        OAuth,
        UserAccessToken,
    }

    let auth_kinds = [
        AuthKind::None,
        AuthKind::Basic,
        AuthKind::PersonalToken,
        AuthKind::App,
        AuthKind::OAuth,
        AuthKind::UserAccessToken,
    ];

    let auth_labels: Vec<_> = auth_kinds
        .iter()
        .map(|kind| match kind {
            AuthKind::None => "No authentication",
            AuthKind::Basic => "Basic HTTP authentication (username:password)",
            AuthKind::PersonalToken => "Authenticate using a Github personal access token",
            AuthKind::App => "Authenticate as a Github App",
            AuthKind::OAuth => "Authenticate as a Github OAuth App",
            AuthKind::UserAccessToken => "Authenticate using a User Access Token",
        })
        .collect();

    let authentication = Select::new("Authenticate?", auth_labels.clone()).prompt()?;

    let index = auth_labels
        .into_iter()
        .position(|label| label == authentication)
        .ok_or_else(|| anyhow::anyhow!("Selected authentication method not found in list"))?;

    match auth_kinds[index] {
        AuthKind::None => request_no_auth(),
        AuthKind::Basic => request_basic_auth(),
        AuthKind::PersonalToken => request_personal_token(),
        AuthKind::App => request_app_auth(),
        AuthKind::OAuth => request_oauth(),
        AuthKind::UserAccessToken => request_user_access_token(),
    }
}

fn request_no_auth() -> anyhow::Result<GithubAuthentication> {
    Ok(GithubAuthentication::None)
}

fn request_basic_auth() -> anyhow::Result<GithubAuthentication> {
    let username = Password::new("Username:")
        .with_display_toggle_enabled()
        .with_display_mode(PasswordDisplayMode::Hidden)
        .prompt()?;
    let password = Password::new("Password:")
        .with_display_toggle_enabled()
        .with_display_mode(PasswordDisplayMode::Hidden)
        .prompt()?;

    Ok(GithubAuthentication::Basic { username, password })
}

fn request_personal_token() -> anyhow::Result<GithubAuthentication> {
    let personal_token = {
        let string = Password::new("Personal token:")
            .with_display_toggle_enabled()
            .with_display_mode(PasswordDisplayMode::Hidden)
            .prompt()
            .map_err(anyhow::Error::from)?;

        SecretString::from(string)
    };

    Ok(GithubAuthentication::PersonalToken(personal_token))
}

fn request_app_auth() -> anyhow::Result<GithubAuthentication> {
    let app_id = {
        let string = Password::new("App ID:")
            .with_display_toggle_enabled()
            .with_display_mode(PasswordDisplayMode::Hidden)
            .prompt()?;

        u64::from_str(&string).map(AppId::from)
    }?;

    let key = {
        let string = Password::new("Encoding key:")
            .with_display_toggle_enabled()
            .with_display_mode(PasswordDisplayMode::Hidden)
            .prompt()?;

        EncodingKey::from_base64_secret(&string)
    }?;

    Ok(GithubAuthentication::App(AppAuth { app_id, key }))
}

fn request_oauth() -> anyhow::Result<GithubAuthentication> {
    let access_token = {
        let string = Password::new("Access token:")
            .with_display_toggle_enabled()
            .with_display_mode(PasswordDisplayMode::Hidden)
            .prompt()
            .map_err(anyhow::Error::from)?;

        SecretString::from(string)
    };

    let token_type = Text::new("Token type:").prompt()?;

    let scope = {
        let string = Text::new("Scope (comma-separated list):").prompt()?;

        string.split(',').map(|s| s.to_owned()).collect::<Vec<_>>()
    };

    Ok(GithubAuthentication::OAuth(octocrab::auth::OAuth {
        access_token,
        expires_in: None,
        refresh_token_expires_in: None,
        refresh_token: None,
        scope,
        token_type,
    }))
}

// Note: Fine-grained personal access tokens may need to be
// explicitly allowed for organizations and their repositories.
fn request_user_access_token() -> anyhow::Result<GithubAuthentication> {
    let personal_token = {
        let string = Password::new("User access token:")
            .with_display_toggle_enabled()
            .with_display_mode(PasswordDisplayMode::Hidden)
            .prompt()
            .map_err(anyhow::Error::from)?;

        SecretString::from(string)
    };

    Ok(GithubAuthentication::UserAccessToken(personal_token))
}
