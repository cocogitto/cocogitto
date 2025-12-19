use std::fmt::Debug;

pub mod github;

pub trait GitProvider: Debug {
    fn get_commit_contributors(
        &self,
        repo: &str,
        org: &str,
        sha: &str,
    ) -> reqwest::Result<Committers>;
}

#[derive(Debug, PartialEq, Eq)]
pub struct Committers {
    pub author: Option<String>,
    pub committer: Option<String>,
}
