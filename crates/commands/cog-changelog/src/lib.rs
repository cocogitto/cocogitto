use cocogitto_changelog::template::{RemoteContext, Template};
use cocogitto_changelog::{get_changelog, get_changelog_at_tag};
use cog_command::CogCommand;

pub struct CogChangelogCommand {
    pub pattern: Option<String>,
    pub at: Option<String>,
    pub template: Option<String>,
    pub remote: Option<String>,
    pub owner: Option<String>,
    pub repository: Option<String>,
}

impl CogCommand for CogChangelogCommand {
    fn execute(self) -> anyhow::Result<()> {
        let repository = &Self::repository()?;
        let path = Self::default_path()?;
        let settings = Self::settings(path.as_path())?;

        let template = settings.changelog.template.as_ref();

        let context =
            RemoteContext::try_new(self.remote, self.repository, self.owner).or_else(|| {
                let remote = settings.changelog.remote.as_ref().cloned();
                let repository = settings.changelog.repository.as_ref().cloned();
                let owner = settings.changelog.owner.as_ref().cloned();
                RemoteContext::try_new(remote, repository, owner)
            });

        let template = self.template.as_ref().or(template);
        let template = if let Some(template) = template {
            Template::from_arg(template, context)?
        } else {
            Template::default()
        };

        let pattern = self.pattern.as_deref().unwrap_or("..");
        let allowed_commits = &settings.allowed_commit_types();
        let omitted_commits = &settings.commit_omitted_from_changelog();
        let changelog_titles = &settings.changelog_titles();
        let usernames = &settings.commit_usernames();

        let result = match self.at.as_ref() {
            Some(at) => get_changelog_at_tag(
                repository,
                at,
                template,
                allowed_commits,
                omitted_commits,
                changelog_titles,
                usernames,
            )?,
            None => {
                let changelog = get_changelog(
                    repository,
                    pattern,
                    allowed_commits,
                    omitted_commits,
                    changelog_titles,
                    usernames,
                )?;
                changelog.into_markdown(template)?
            }
        };

        println!("{result}");
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use assert_cmd::Command;
    use cmd_lib::run_cmd;
    use sealed_test::prelude::*;
    use speculoos::prelude::*;

    use cocogitto_changelog::get_changelog;
    use cocogitto_config::Settings;

    use cocogitto_test_helpers::*;

    #[sealed_test]
    fn getting_changelog_from_tags_should_produce_the_same_range_either_from_tags_or_from_commits(
    ) -> Result<()> {
        // Arrange
        let repository = git_init_no_gpg()?;

        git_commit("feat: feature 1")?;
        let sha_0_1 = git_commit("feat: feature 2")?;
        git_tag("0.1.0")?;
        git_commit("feat: feature 3")?;
        git_commit("feat: feature 4")?;

        Command::cargo_bin("cog")?
            .arg("bump")
            .arg("--auto")
            .assert()
            .success();

        let head = git_log_head_sha()?;

        run_cmd!(
            git log --graph --abbrev-commit;
        )
        .unwrap();

        let settings = Settings::default();
        let allowed_commits = &settings.allowed_commit_types();
        let omitted_commits = &settings.commit_omitted_from_changelog();
        let changelog_titles = &settings.changelog_titles();
        let usernames = &settings.commit_usernames();

        // Act
        let changelog_from_commit_range = get_changelog(
            &repository,
            &format!("{sha_0_1}..{head}"),
            allowed_commits,
            omitted_commits,
            changelog_titles,
            usernames,
        )?;
        let changelog_tag_range = get_changelog(
            &repository,
            "0.1.0..0.2.0",
            allowed_commits,
            omitted_commits,
            changelog_titles,
            usernames,
        )?;
        let at_tag = get_changelog(
            &repository,
            "..0.2.0",
            allowed_commits,
            omitted_commits,
            changelog_titles,
            usernames,
        )?;

        let commit_range_oids: Vec<String> = changelog_from_commit_range
            .commits
            .into_iter()
            .map(|commit| commit.commit.conventional.summary)
            .collect();

        let tag_range_oids: Vec<String> = changelog_tag_range
            .commits
            .into_iter()
            .map(|commit| commit.commit.conventional.summary)
            .collect();

        let at_tag_oids: Vec<String> = at_tag
            .commits
            .into_iter()
            .map(|commit| commit.commit.conventional.summary)
            .collect();

        // Assert
        asserting!("Changelog commits generated from a commit range should be equivalent to when generated from the same tag range")
            .that(&commit_range_oids)
            .is_equal_to(&tag_range_oids);

        asserting!("Changelog commits generated from a commit range should be equivalent to when generated from the same tag")
            .that(&commit_range_oids)
            .is_equal_to(&at_tag_oids);

        Ok(())
    }

    #[sealed_test]
    fn from_commit_should_be_drained() -> Result<()> {
        // Arrange
        let repository = git_init_no_gpg()?;

        git_commit("feat: feature 1")?;
        git_commit("feat: feature 2")?;
        git_tag("0.1.0")?;
        git_commit("feat: feature 3")?;
        let unttaged_sha = git_commit("feat: feature 4")?;

        Command::cargo_bin("cog")?
            .arg("bump")
            .arg("--auto")
            .assert()
            .success();

        let head = git_log_head_sha()?;

        let settings = Settings::default();
        let allowed_commits = &settings.allowed_commit_types();
        let omitted_commits = &settings.commit_omitted_from_changelog();
        let changelog_titles = &settings.changelog_titles();
        let usernames = &settings.commit_usernames();

        // Act
        let changelog_from_commit_range = get_changelog(
            &repository,
            &format!("{unttaged_sha}..{head}"),
            allowed_commits,
            omitted_commits,
            changelog_titles,
            usernames,
        )?;

        let commit_range_oids: Vec<String> = changelog_from_commit_range
            .commits
            .into_iter()
            .map(|commit| commit.commit.oid)
            .collect();

        // Assert
        asserting!("Changelog commits generated from a commit range should be equivalent to when generated from the same tag range")
            .that(&commit_range_oids)
            .is_equal_to(vec![head]);

        Ok(())
    }
}
