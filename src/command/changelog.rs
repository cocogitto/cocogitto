use crate::conventional::changelog::release::Release;
use crate::conventional::changelog::template::MonoRepoContext;
use crate::conventional::changelog::template::PackageBumpContext;
use crate::conventional::changelog::template::Template;

use crate::conventional::changelog::ReleaseType;
use crate::CocoGitto;
use crate::SETTINGS;
use anyhow::anyhow;
use anyhow::Result;

impl CocoGitto {
    /// ## Get a changelog between two oids
    /// - `from` default value:latest tag or else first commit
    /// - `to` default value:`HEAD` or else first commit
    pub fn get_changelog(&self, pattern: &str, _with_child_releases: bool) -> Result<Release> {
        let commit_range = self.repository.revwalk(pattern)?;
        Release::try_from(commit_range).map_err(Into::into)
    }

    pub fn get_monorepo_changelog(&self, pattern: &str, template: Template) -> Result<String> {
        let mut packages = vec![];

        let package_data: Vec<(&String, String)> = SETTINGS
            .packages
            .iter()
            .map(|(name, p)| (name, p.path.to_string_lossy().to_string()))
            .collect();

        for (package_name, package_path) in package_data.iter() {
            let range = self
                .repository
                .get_commit_range_for_package(pattern, package_name)?;

            let from = Some(range.from_oid());

            let version = range.to_oid();

            let context = PackageBumpContext {
                package_name,
                package_path,
                version,
                from,
            };

            packages.push(context);
        }
        let context = MonoRepoContext {
            package_lock: false,
            packages,
        };

        let commit_range = self
            .repository
            .get_commit_range_for_monorepo_global(pattern)?;

        let changelog = Release::try_from(commit_range)?;
        changelog
            .into_markdown(template, ReleaseType::MonoRepo(context))
            .map_err(Into::into)
    }

    pub fn get_changelog_at_tag(&self, tag: &str, template: Template) -> Result<String> {
        let changelog = self.get_changelog(tag, false)?;

        changelog
            .into_markdown(template, ReleaseType::Standard)
            .map_err(|err| anyhow!(err))
    }
}
