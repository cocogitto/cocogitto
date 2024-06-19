use cocogitto::CogCommand;
use cocogitto_changelog::template::{RemoteContext, Template};
use cocogitto_changelog::{get_changelog, get_changelog_at_tag};

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
