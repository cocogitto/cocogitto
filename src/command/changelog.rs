use crate::conventional::changelog::context::MonoRepoContext;
use crate::conventional::changelog::context::PackageBumpContext;
use crate::conventional::changelog::release::Release;
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
    pub fn get_release(&self, pattern: &str) -> Result<Release> {
        let commit_range = self.repository.revwalk(pattern)?;
        Release::try_from(commit_range).map_err(Into::into)
    }

    pub fn get_changelog_at_tag(&self, tag: &str, template: Template) -> Result<String> {
        let changelog = self.get_release(tag)?;

        changelog
            .render(template, ReleaseType::Standard)
            .map_err(|err| anyhow!(err))
    }

    pub fn get_monorepo_release(
        &self,
        pattern: &str,
        unified: bool,
    ) -> Result<(MonoRepoContext, Release)> {
        let mut packages = vec![];

        let mut package_data: Vec<(&String, String)> = SETTINGS
            .packages
            .iter()
            .map(|(name, p)| (name, p.path.to_string_lossy().to_string()))
            .collect();
        package_data.sort_by_key(|&(name, _)| name);

        for (package_name, package_path) in package_data.into_iter() {
            let range = self
                .repository
                .get_commit_range_for_package(pattern, package_name)?;

            if range.is_empty() {
                continue;
            }

            let from = Some(range.from_oid());
            let version = range.to_oid();
            let context = PackageBumpContext {
                package_name: package_name.clone(),
                package_path: package_path.clone(),
                version,
                from,
            };

            packages.push(context);
        }
        let context = MonoRepoContext {
            package_lock: false,
            packages,
        };

        let commit_range = if unified {
            self.repository.revwalk(pattern)?
        } else {
            self.repository
                .get_commit_range_for_monorepo_global(pattern)?
        };

        let changelog = Release::try_from(commit_range)?;
        Ok((context, changelog))
    }

    pub fn get_monorepo_changelog(
        &self,
        pattern: &str,
        template: Template,
        unified: bool,
    ) -> Result<String> {
        let (context, release) = self.get_monorepo_release(pattern, unified)?;

        release
            .render(template, ReleaseType::MonoRepo(context))
            .map_err(Into::into)
    }
}
