use crate::command::bump::{ensure_tag_is_greater_than_previous, tag_or_fallback_to_zero};
use crate::conventional::changelog::release::Release;
use crate::conventional::version::VersionIncrement;
use crate::git::oid::OidOf;
use crate::git::revspec::RevspecPattern;
use crate::git::tag::Tag;
use crate::hook::HookVersion;
use crate::settings::HookType;
use crate::{CocoGitto, SETTINGS};
use anyhow::{bail, Result};
use colored::*;
use log::info;
use semver::Prerelease;
use std::path::{Path, PathBuf};

impl CocoGitto {
    pub fn create_monorepo_version(
        &mut self,
        pre_release: Option<&str>,
        hooks_config: Option<&str>,
        dry_run: bool,
    ) -> Result<()> {
        self.pre_bump_checks()?;
        let mut package_bumps = vec![];

        for (package_name, package) in &SETTINGS.packages {
            let old = self.repository.get_latest_package_tag(package_name);
            let old = tag_or_fallback_to_zero(old)?;
            info!("Package {}, current version {old}  ", package_name.bold());
            let mut next_version = old.bump(VersionIncrement::Auto, &self.repository)?;
            ensure_tag_is_greater_than_previous(&old, &next_version)?;

            if let Some(pre_release) = pre_release {
                next_version.version.pre = Prerelease::new(pre_release)?;
            }

            let tag = Tag::create(next_version.version, Some(package_name.to_string()));

            if dry_run {
                print!("{}", tag);
                continue;
            }

            let pattern = self.get_revspec_for_tag(&old)?;

            let changelog_start = if old.is_zero() {
                None
            } else {
                Some(OidOf::Tag(old))
            };

            let Some(changelog) = self.get_changelog_with_target_package_version(pattern, changelog_start, tag.clone(), package)? else {
                println!("\t No commit found to bump package, skipping.");
                continue;
            };

            let path = package.changelog_path();
            let template = SETTINGS.get_changelog_template()?;
            changelog.write_to_file(path, template)?;

            let current = self
                .repository
                .get_latest_package_tag(package_name)
                .map(HookVersion::new)
                .ok();

            let next_version = HookVersion::new(tag.clone());

            let hook_result = self.run_hooks(
                HookType::PreBump,
                current.as_ref(),
                &next_version,
                hooks_config,
                Some(package),
            );

            self.repository.add_all()?;

            // Hook failed, we need to stop here and reset
            // the repository to a clean state
            if let Err(err) = hook_result {
                self.stash_failed_version(&tag, err)?;
            }

            package_bumps.push((package_name, package, current, next_version, tag));
        }

        if package_bumps.is_empty() {
            bail!("Nothing to bump");
        }

        let mut meta_bump = None;
        for (_, _, old, _, new) in &package_bumps {
            if let Some(old) = old.as_ref() {
                let old = &old.prefixed_tag.version;
                let new = &new.version;
                if old.major > new.major {
                    meta_bump = Some(VersionIncrement::Major)
                } else if old.minor > new.minor {
                    if meta_bump != Some(VersionIncrement::Major)
                        || meta_bump != Some(VersionIncrement::Minor)
                    {
                        meta_bump = Some(VersionIncrement::Minor)
                    }
                } else if meta_bump != Some(VersionIncrement::Major)
                    || meta_bump != Some(VersionIncrement::Minor)
                    || meta_bump != Some(VersionIncrement::Patch)
                {
                    meta_bump = Some(VersionIncrement::Patch)
                }
            }
        }

        // Generate a meta changelog, aggregating new changelog versions
        // and commits that does not belong to any package
        let current_tag = self.repository.get_latest_tag();
        let current_tag = tag_or_fallback_to_zero(current_tag)?;
        let mut tag = current_tag.bump(meta_bump.unwrap(), &self.repository)?;
        ensure_tag_is_greater_than_previous(&current_tag, &tag)?;
        if let Some(pre_release) = pre_release {
            tag.version.pre = Prerelease::new(pre_release)?;
        }
        let tag = Tag::create(tag.version, None);
        let pattern = self.get_revspec_for_tag(&current_tag)?;

        let changelog_start = if current_tag.is_zero() {
            None
        } else {
            Some(OidOf::Tag(current_tag.clone()))
        };

        let _changelog = self.get_meta_changelog(pattern, changelog_start, tag)?;
        let _template = SETTINGS.get_changelog_template()?;
        // changelog.write_to_file(path, template)?;

        self.repository.commit("chore(version): {tag}", false)?;

        for (package_name, package, current, next_version, tag) in package_bumps {
            self.repository.create_tag(&tag)?;

            self.run_hooks(
                HookType::PostBump,
                current.as_ref(),
                &next_version,
                hooks_config,
                Some(package),
            )?;

            let current = current
                .map(|current| current.prefixed_tag.to_string())
                .unwrap_or_else(|| "...".to_string());
            let bump = format!("{} -> {}", current, next_version.prefixed_tag).green();
            info!("Bumped package {package_name} version: {}", bump);
        }

        Ok(())
    }

    /// Used for monorepo package bump. Get all commits that modify at least one path that does not
    /// belong to a monorepo package.
    /// Target version is not created yet when generating the changelog.
    fn get_meta_changelog(
        &self,
        pattern: RevspecPattern,
        starting_tag: Option<OidOf>,
        target_tag: Tag,
    ) -> Result<Option<Release>> {
        let package_paths: Vec<&PathBuf> = SETTINGS
            .packages
            .values()
            .map(|package| &package.path)
            .collect();

        let filter = |path: &Path| {
            package_paths
                .iter()
                .any(|package_path| !path.starts_with(package_path))
        };

        let mut release = self
            .repository
            .get_commit_range_filtered(starting_tag, &pattern, filter)?
            .map(Release::from);

        if let Some(release) = &mut release {
            release.version = OidOf::Tag(target_tag);
        }

        Ok(release)
    }
}
