#![feature(drain_filter)]
#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate serde_derive;

mod changelog;
mod commit;
pub mod repository;
pub mod settings;

use crate::changelog::Changelog;
use crate::repository::Repository;
use crate::settings::Settings;
use anyhow::Result;
use chrono::Utc;
use colored::*;
use commit::Commit;
use git2::{Oid, Repository as Git2Repository};

pub struct CocoGitto {
    pub settings: Settings,
    repository: Repository,
}

impl CocoGitto {
    pub fn get() -> Result<Self> {
        let repository = Repository::open()?;
        let settings = Settings::get(&repository)?;

        Ok(CocoGitto {
            settings,
            repository,
        })
    }

    pub fn commit_types(&self) -> Vec<String> {
        let mut commit_types: Vec<String> = self
            .settings
            .commit_types
            .iter()
            .map(|(key, _)| key)
            .cloned()
            .collect();

        commit_types.extend_from_slice(&[
            "feat".to_string(),
            "fix".to_string(),
            "chore".to_string(),
            "revert".to_string(),
            "perf".to_string(),
            "docs".to_string(),
            "style".to_string(),
            "refactor".to_string(),
            "test".to_string(),
            "build".to_string(),
            "ci".to_string(),
        ]);

        commit_types
    }

    pub fn get_repository(&self) -> &Git2Repository {
        &self.repository.0
    }

    pub fn check() -> () {
        todo!()
    }

    pub fn version() -> () {
        todo!()
    }

    pub fn get_changelog(&self, from: Option<&str>, to: Option<&str>) -> anyhow::Result<String> {
        let from = self.resolve_from_arg(from)?;
        let to = self.resolve_to_arg(to)?;

        let commits = self.get_changelog_from_oid_range(from, to)?;
        let date = Utc::now().naive_utc().date().to_string();

        let mut changelog = Changelog {
            from,
            to,
            date,
            commits,
        };

        Ok(changelog.tag_diff_to_markdown())
    }

    // TODO : revparse
    fn resolve_to_arg(&self, to: Option<&str>) -> Result<Oid> {
        if let Some(to) = to {
            if to.contains(".") {
                self.repository.resolve_lightweight_tag(to)
            } else {
                Oid::from_str(to)
            }
            .to_owned()
        } else {
            self.repository
                .get_head_commit_oid()
                .unwrap_or(self.repository.get_first_commit()?)
        }
    }

    // TODO : revparse
    fn resolve_from_arg(&self, from: Option<&str>) -> Result<Oid> {
        if let Some(from) = from {
            if from.contains(".") {
                self.repository.resolve_lightweight_tag(from)
            } else {
                Oid::from_str(from)
            }
            .to_owned()
        } else {
            self.repository
                .get_latest_tag()
                .unwrap_or(self.repository.get_first_commit()?)
        }
    }

    fn get_changelog_from_oid_range(&self, from: Oid, to: Oid) -> Result<Vec<Commit>> {
        println!("get changelog from {} to {}", from, to);
        // Ensure commit exists
        let repository = self.get_repository();
        repository.find_commit(from)?;
        repository.find_commit(to)?;

        let mut revwalk = repository.revwalk()?;
        revwalk.push(to)?;
        revwalk.push(from)?;

        let mut commits = vec![];

        for oid in revwalk {
            let oid = oid?;

            if oid == from {
                break;
            }

            let commit = repository.find_commit(oid)?;
            match Commit::from_git_commit(commit) {
                Ok(commit) => commits.push(commit),
                Err(err) => {
                    let err = format!("{}", err).red();
                    eprintln!("{}", err);
                }
            }
        }

        Ok(commits)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn should_open_repo() {}
}
