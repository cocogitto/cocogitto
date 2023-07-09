use crate::conventional::changelog::release::Release;
use crate::conventional::changelog::template::Template;
use crate::git::revspec::RevspecPattern;
use crate::CocoGitto;
use anyhow::anyhow;
use anyhow::Result;
use std::str::FromStr;

impl CocoGitto {
    /// ## Get a changelog between two oids
    /// - `from` default value:latest tag or else first commit
    /// - `to` default value:`HEAD` or else first commit
    pub fn get_changelog(
        &self,
        pattern: RevspecPattern,
        with_child_releases: bool,
    ) -> Result<Release> {
        if with_child_releases {
            self.repository
                .get_release_range(pattern)
                .map_err(Into::into)
        } else {
            let commit_range = self.repository.get_commit_range(&pattern)?;

            Ok(Release::from(commit_range))
        }
    }

    pub fn get_changelog_at_tag(&self, tag: &str, template: Template) -> Result<String> {
        let pattern = format!("..{tag}");
        let pattern = RevspecPattern::from_str(pattern.as_str())?;
        let changelog = self.get_changelog(pattern, false)?;

        changelog
            .into_markdown(template)
            .map_err(|err| anyhow!(err))
    }
}
