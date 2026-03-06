use crate::conventional::changelog::context::RemoteContext;
use crate::conventional::changelog::error::ChangelogError;
use crate::conventional::changelog::template::Template;
use cocogitto_settings::Settings;

pub mod bump;
pub mod changelog;
pub mod commit;
pub(crate) mod error;
pub mod version;

/// Creates a template context for remote changelog generation.
///
/// # Returns
///
/// * `Option<RemoteContext>` - Context for remote template rendering, or None if not configured
pub fn get_template_context(settings: &Settings) -> Option<RemoteContext> {
    let remote = settings.changelog.remote.as_ref().cloned();
    let repository = settings.changelog.repository.as_ref().cloned();
    let owner = settings.changelog.owner.as_ref().cloned();

    RemoteContext::try_new(remote, repository, owner)
}

/// Gets the changelog template for the main repository.
///
/// # Returns
///
/// * `Result<Template, ChangelogError>` - The changelog template
pub fn get_changelog_template(settings: &Settings) -> Result<Template, ChangelogError> {
    let context = get_template_context(settings);
    let template = settings.changelog.template.as_deref().unwrap_or("default");

    // TODO: there should be a unified settings
    Template::from_arg(template, context, false)
}

/// Gets the changelog template for package changelogs in monorepos.
///
/// # Returns
///
/// * `Result<Template, ChangelogError>` - The package changelog template
pub fn get_package_changelog_template(settings: &Settings) -> Result<Template, ChangelogError> {
    let context = get_template_context(settings);
    let template = settings
        .changelog
        .package_template
        .as_deref()
        .unwrap_or("package_default");

    let template = match template {
        "remote" => "package_remote",
        "full_hash" => "package_full_hash",
        template => template,
    };

    // TODO: there should be a unified settings
    Template::from_arg(template, context, false)
}

/// Gets the changelog template for monorepo changelogs.
///
/// # Returns
///
/// * `Result<Template, ChangelogError>` - The monorepo changelog template
pub fn get_monorepo_changelog_template(settings: &Settings) -> Result<Template, ChangelogError> {
    let context = get_template_context(settings);
    let template = settings
        .changelog
        .template
        .as_deref()
        .unwrap_or("monorepo_default");

    let template = match template {
        "remote" => "monorepo_remote",
        "full_hash" => "monorepo_full_hash",
        template => template,
    };

    // TODO: there should be a unified settings
    Template::from_arg(template, context, false)
}
