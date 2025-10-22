mod github;

pub trait GitProvider {
    fn get_commit_contributors(
        &self,
        repo: &str,
        org: &str,
        sha: &str,
    ) -> reqwest::Result<Vec<String>>;
}
