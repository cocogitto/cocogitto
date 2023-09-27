use crate::git::repository::Repository;

use crate::{Tag, TagError};

impl Repository {
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
    use std::str::FromStr;

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
        repo.commit("chore: init", false, false)?;

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
            repo.get_commit_range_for_package(&RevspecPattern::from_str("..HEAD")?, "two")?;

        // Assert
        assert_that!(range)
            .map(|range| &range.commits)
            .has_length(2);

        Ok(())
    }
}
