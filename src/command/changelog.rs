use crate::conventional::changelog::release::Release;
use crate::CocoGitto;
use anyhow::Result;

impl CocoGitto {
    /// ## Get a changelog between two oids
    /// - `from` default value:latest tag or else first commit
    /// - `to` default value:`HEAD` or else first commit
    pub fn get_changelog(&self, pattern: &str, _with_child_releases: bool) -> Result<Release> {
        let commit_range = self.repository.revwalk(pattern)?;
        Release::try_from(commit_range).map_err(Into::into)
    }

    pub fn get_changelog_for_package(
        &self,
        pattern: &str,
        package: &str,
        _with_child_releases: bool,
    ) -> Result<Release> {
        let commit_range = self
            .repository
            .get_commit_range_for_package(pattern, package)?;
        Release::try_from(commit_range).map_err(Into::into)
    }
}
