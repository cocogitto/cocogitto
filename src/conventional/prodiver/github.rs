use serde::Deserialize;

use crate::conventional::prodiver::{Committers, GitProvider};

#[derive(Debug)]
pub struct GitHubProvider {
    client: reqwest::blocking::Client,
}

#[derive(Deserialize, Debug)]
pub struct GitHubAuthors {
    committer: GitHubCommiter,
    author: GitHubCommiter,
}

#[derive(Deserialize, Debug)]
pub struct GitHubCommiter {
    pub login: Option<String>,
}

impl Default for GitHubProvider {
    fn default() -> Self {
        Self {
            client: reqwest::blocking::Client::builder()
                .user_agent("cocogitto")
                .build()
                .expect("Failed to build client"),
        }
    }
}

impl GitProvider for GitHubProvider {
    fn get_commit_contributors(
        &self,
        repo: &str,
        org: &str,
        sha: &str,
    ) -> reqwest::Result<Committers> {
        let uri = format!("https://api.github.com/repos/{org}/{repo}/commits/{sha}");
        let response = self.client.get(uri).send()?.json::<GitHubAuthors>()?;

        Ok(Committers {
            author: response.author.login,
            committer: response.committer.login,
        })
    }
}

#[cfg(test)]
mod tests {
    use speculoos::{assert_that, result::ContainingResultAssertions};

    use super::*;
    use crate::conventional::prodiver::GitProvider;

    #[test]
    fn test_get_contributors() {
        let provider = GitHubProvider::default();

        let result = provider.get_commit_contributors(
            "cocogitto",
            "cocogitto",
            "5628b0c0071acba95dbec603d171dc9c92cf5b19",
        );

        assert_that!(result).is_ok_containing(Committers {
            author: Some("oknozor".to_string()),
            committer: Some("ba-lidnner".to_string()),
        });
    }
}
