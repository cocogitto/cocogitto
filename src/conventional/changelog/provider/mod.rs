use std::fmt::Debug;

use crate::settings;

pub mod github;

pub trait GitProvider: Debug {
    fn get_commit_contributors(
        &self,
        repo: &str,
        org: &str,
        sha: &str,
    ) -> reqwest::Result<Committers>;
}

impl From<settings::GitProvider> for Box<dyn GitProvider> {
    fn from(provider: settings::GitProvider) -> Self {
        match provider {
            settings::GitProvider::Github => Box::new(github::GitHubProvider::default()),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Committers {
    pub author: Option<String>,
    pub committer: Option<String>,
}
