use crate::git::repository::Repository;
use crate::git::revspec::CommitRange;

use crate::{Git2Error, OidOf, RevspecPattern, Tag, TagError};
use std::path::Path;

impl Repository {
    /// Get commits from latest tag and return a map of commit ranges by their respective packages.
    pub fn get_commit_range_filtered(
        &self,
        start: Option<OidOf>,
        pattern: &RevspecPattern,
        path_filter: impl Fn(&Path) -> bool,
    ) -> Result<Option<CommitRange>, Git2Error> {
        let range = self.get_commit_range(pattern)?;

        let mut commits = vec![];

        for commit in range.commits {
            let parent = commit.parent(0)?.id().to_string();
            let t1 = self
                .tree_to_treeish(Some(&parent))?
                .expect("Failed to get parent tree");

            let t2 = self
                .tree_to_treeish(Some(&commit.id().to_string()))?
                .expect("Failed to get commit tree");

            let diff = self.0.diff_tree_to_tree(t1.as_tree(), t2.as_tree(), None)?;

            for delta in diff.deltas() {
                if let Some(old) = delta.old_file().path() {
                    if path_filter(old) {
                        commits.push(commit);
                        break;
                    }
                }

                if let Some(new) = delta.new_file().path() {
                    if path_filter(new) {
                        commits.push(commit);
                        break;
                    }
                }
            }
        }

        if !commits.is_empty() {
            Ok(Some(CommitRange {
                from: start.unwrap_or(OidOf::Other(commits.first().unwrap().id())),
                // Safe unwrap, matches are not empty
                to: OidOf::Other(commits.last().unwrap().id()),
                commits,
            }))
        } else {
            Ok(None)
        }
    }

    /// Get the latest SemVer tag for a given monorepo package.
    pub fn get_latest_package_tag(&self, package_prefix: &str) -> Result<Tag, TagError> {
        let tags: Vec<Tag> = self.all_tags()?;

        tags.into_iter()
            .filter(|tag| {
                tag.package
                    .as_ref()
                    .map(|package| package == package_prefix)
                    .unwrap_or_default()
            })
            .max()
            .ok_or(TagError::NoTag)
    }
}

#[cfg(test)]
mod test {
    use crate::{Repository, RevspecPattern};
    use anyhow::Result;
    use cmd_lib::run_cmd;
    use indoc::formatdoc;
    use sealed_test::prelude::*;
    use speculoos::prelude::*;
    use std::path::PathBuf;

    #[sealed_test]
    fn get_repo_packages() -> Result<()> {
        // Arrange
        let settings = formatdoc!(
            "
            [packages.one]
            path = \"one\"
            changelog_path = \"one/CHANGELOG.md\"

            [packages.two]
            path = \"two\"
            changelog_path = \"two/CHANGELOG.md\"
            "
        );

        run_cmd!(
            git init -b master;
            echo $settings > cog.toml;
            git add .;
        )?;

        let repo = Repository::open(".")?;
        repo.commit("chore: init", false)?;

        run_cmd!(
            mkdir one;
            echo "one" > one/file;
            git add .;
            git commit -m "feat: package one";
            mkdir two;
            echo "two" > two/file;
            git add .;
            git commit -m "feat: package two";
            echo "two" > two/file2;
            git add .;
            git commit -m "feat: more changes to two";
        )?;

        // Act
        let range =
            repo.get_commit_range_filtered(None, &RevspecPattern::from("..HEAD"), |path| {
                path.starts_with(PathBuf::from("two"))
            })?;

        // Assert
        assert_that!(range)
            .is_some()
            .map(|range| &range.commits)
            .has_length(2);

        Ok(())
    }
}
