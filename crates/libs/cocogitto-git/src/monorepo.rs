use crate::tag::TagLookUpOptions;
use crate::Repository;
use cocogitto_tag::error::TagError;
use cocogitto_tag::Tag;

impl Repository {
    /// Get the latest SemVer tag for a given monorepo package.
    pub fn get_latest_package_tag(&self, package: &str) -> Result<Tag, TagError> {
        let tags: Vec<Tag> = self.tag_lookup(TagLookUpOptions::package(package))?;

        tags.into_iter().max().ok_or(TagError::NoTag)
    }
}

#[cfg(test)]
mod test {

    use crate::test::git_init_no_gpg;
    use anyhow::Result;
    use cmd_lib::run_cmd;
    use indoc::formatdoc;
    use sealed_test::prelude::*;
    use speculoos::prelude::*;

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

        let repo = git_init_no_gpg()?;
        run_cmd!(
            echo $settings > cog.toml;
            git add .;
        )?;

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
        let range = repo.get_commit_range_for_package("..HEAD", "two")?;
        let range = range.into_iter().collect::<Vec<_>>();

        // Assert
        assert_that!(range).has_length(2);

        Ok(())
    }
}
