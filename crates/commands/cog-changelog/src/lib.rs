use cocogitto::command::changelog::{get_changelog, get_changelog_at_tag, get_template_context};
use cocogitto::CogCommand;
use cocogitto_changelog::template::{RemoteContext, Template};

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
        let settings = Self::settings()?;

        let template = settings.changelog.template.as_ref();

        let context = RemoteContext::try_new(self.remote, self.repository, self.owner)
            .or_else(get_template_context);
        let template = self.template.as_ref().or(template);
        let template = if let Some(template) = template {
            Template::from_arg(template, context)?
        } else {
            Template::default()
        };

        // TODO: fallback to tag here
        let pattern = self.pattern.as_deref().unwrap_or("..");
        let result = match self.at.as_ref() {
            Some(at) => get_changelog_at_tag(repository, at, template)?,
            None => {
                let changelog = get_changelog(repository, pattern, true)?;
                changelog.into_markdown(template)?
            }
        };

        println!("{result}");
        Ok(())
    }
}
