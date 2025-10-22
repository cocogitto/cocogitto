use serde::Deserialize;

use crate::conventional::prodiver::GitProvider;

pub struct GithubProvider {
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

impl Default for GithubProvider {
    fn default() -> Self {
        Self {
            client: reqwest::blocking::Client::builder()
                .user_agent("cocogitto")
                .build()
                .expect("Failed to build client"),
        }
    }
}

impl GitProvider for GithubProvider {
    fn get_commit_contributors(
        &self,
        repo: &str,
        org: &str,
        sha: &str,
    ) -> reqwest::Result<Vec<String>> {
        let uri = format!("https://api.github.com/repos/{org}/{repo}/commits/{sha}");
        let response = self.client.get(uri).send()?.json::<GitHubAuthors>()?;
        let mut authors = Vec::with_capacity(2);

        if let Some(login) = response.committer.login {
            authors.push(format!("@{}", login));
        }

        if let Some(login) = response.author.login {
            authors.push(format!("@{}", login));
        }

        Ok(authors)
    }
}

#[cfg(test)]
mod tests {
    use speculoos::{assert_that, result::ContainingResultAssertions};

    use super::*;
    use crate::conventional::prodiver::GitProvider;

    #[test]
    fn test_get_contributors() {
        let provider = GithubProvider::default();

        let result = provider.get_commit_contributors(
            "cocogitto",
            "cocogitto",
            "5628b0c0071acba95dbec603d171dc9c92cf5b19",
        );

        assert_that!(result)
            .is_ok_containing(&vec!["@oknozor".to_string(), "@ba-lindner".to_string()]);
    }
}
