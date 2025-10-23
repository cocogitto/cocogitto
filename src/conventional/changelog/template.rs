use crate::conventional::changelog::context::{RemoteContext, ToContext};
use crate::conventional::changelog::error::ChangelogError;
use crate::conventional::changelog::release::{ChangelogFooter, Release};
use crate::conventional::prodiver::github::GitHubProvider;
use crate::conventional::prodiver::GitProvider;

use super::filter;
use std::collections::HashMap;
use std::io;
use std::path::PathBuf;
use tera::{Context, Tera};

const DEFAULT_TEMPLATE: &[u8] = include_bytes!("template/simple");
const DEFAULT_TEMPLATE_NAME: &str = "default";
const REMOTE_TEMPLATE: &[u8] = include_bytes!("template/remote");
const REMOTE_TEMPLATE_NAME: &str = "remote";
const FULL_HASH_TEMPLATE: &[u8] = include_bytes!("template/full_hash");
const FULL_HASH_TEMPLATE_NAME: &str = "full_hash";
const GITHUB_TEMPLATE: &[u8] = include_bytes!("template/github");
const GITHUB_TEMPLATE_NAME: &str = "github";

const PACKAGE_DEFAULT_TEMPLATE: &[u8] = include_bytes!("template/package_simple");
const PACKAGE_DEFAULT_TEMPLATE_NAME: &str = "package_default";
const PACKAGE_REMOTE_TEMPLATE: &[u8] = include_bytes!("template/package_remote");
const PACKAGE_REMOTE_TEMPLATE_NAME: &str = "package_remote";
const PACKAGE_FULL_HASH_TEMPLATE: &[u8] = include_bytes!("template/package_full_hash");
const PACKAGE_FULL_HASH_TEMPLATE_NAME: &str = "package_full_hash";

const MONOREPO_DEFAULT_TEMPLATE: &[u8] = include_bytes!("template/monorepo_simple");
const MONOREPO_DEFAULT_TEMPLATE_NAME: &str = "monorepo_default";
const MONOREPO_REMOTE_TEMPLATE: &[u8] = include_bytes!("template/monorepo_remote");
const MONOREPO_REMOTE_TEMPLATE_NAME: &str = "monorepo_remote";
const MONOREPO_FULL_HASH_TEMPLATE: &[u8] = include_bytes!("template/monorepo_full_hash");
const MONOREPO_FULL_HASH_TEMPLATE_NAME: &str = "monorepo_full_hash";

#[derive(Debug)]
pub struct Template {
    pub tera: Tera,
    pub context: Context,
    pub kind: TemplateKind,
    pub remote_context: Option<RemoteContext>,
    pub git_provider: Option<Box<dyn GitProvider>>,
}

impl Template {
    pub fn from_arg(
        value: &str,
        remote_context: Option<RemoteContext>,
    ) -> Result<Self, ChangelogError> {
        let template = TemplateKind::from_arg(value)?;
        let mut context = Context::default();
        if let Some(remote) = &remote_context {
            context.extend(remote.to_context());
        }

        let mut tera = Tera::default();
        let content = template.get()?;
        let content = String::from_utf8_lossy(content.as_slice());
        tera.add_raw_template(template.name(), content.as_ref())?;
        tera.register_filter("upper_first", filter::upper_first_filter);
        tera.register_filter("unscoped", filter::unscoped);
        tera.register_filter("group_by_type", filter::group_by_type);

        let git_provider: Option<Box<dyn GitProvider>> = match template {
            TemplateKind::Github => Some(Box::new(GitHubProvider::default())),
            _ => None,
        };

        Ok(Template {
            context,
            kind: template,
            tera,
            git_provider,
            remote_context,
        })
    }

    pub fn render(&mut self, mut version: Release) -> Result<String, ChangelogError> {
        let mut contributor_map = HashMap::new();
        let mut release = self.render_release(&mut version, &mut contributor_map)?;
        let mut version = version;
        while let Some(mut previous) = version.previous.map(|v| *v) {
            release.push_str("\n- - -\n\n");
            release.push_str(
                self.render_release(&mut previous, &mut contributor_map)?
                    .as_str(),
            );
            version = previous;
        }

        Ok(release)
    }

    fn render_release(
        &mut self,
        version: &mut Release,
        contributor_map: &mut HashMap<String, String>,
    ) -> Result<String, ChangelogError> {
        if let (Some(git_provider), Some(remote_context)) =
            (self.git_provider.as_ref(), self.remote_context.as_ref())
        {
            let owner = &remote_context.owner;
            let repo = &remote_context.repository;

            for commit in &mut version.commits {
                if let Some(username) = contributor_map.get(&commit.commit.author) {
                    // TODO: add committer
                    commit.author_username = Some(username.clone());
                } else {
                    let github_authors =
                        git_provider.get_commit_contributors(owner, repo, &commit.commit.oid)?;

                    if let Some(author) = github_authors.author {
                        contributor_map.insert(commit.commit.author.clone(), author.clone());
                        commit.author_username = Some(author);
                    }

                    if let Some(committer) = github_authors.committer {
                        contributor_map.insert(commit.commit.committer.clone(), committer);
                    }
                }

                // TODO: handle when co-author not in committer or commit author
                commit.co_authors = commit
                    .commit
                    .conventional
                    .footers
                    .iter()
                    .map(ChangelogFooter::from)
                    .filter_map(|footer| match footer {
                        ChangelogFooter::GithubCoAuthoredBy { user } => Some(user.to_string()),
                        _ => None,
                    })
                    .filter_map(|author| contributor_map.get(&author))
                    .cloned()
                    .collect::<Vec<_>>();
            }
        }

        self.push_context(version);
        self.tera
            .render(self.kind.name(), &self.context)
            .map_err(Into::into)
    }

    pub(crate) fn push_context(&mut self, context: impl ToContext) {
        self.context.extend(context.to_context());
    }

    pub(crate) fn with_context(mut self, context: impl ToContext) -> Self {
        self.context.extend(context.to_context());
        self
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub enum TemplateKind {
    #[default]
    Default,
    FullHash,
    Remote,
    Github,
    PackageDefault,
    PackageFullHash,
    PackageRemote,
    MonorepoDefault,
    MonorepoFullHash,
    MonorepoRemote,
    Custom(PathBuf),
}

impl TemplateKind {
    /// Returns either a predefined template or a custom template
    fn from_arg(value: &str) -> Result<Self, ChangelogError> {
        match value {
            DEFAULT_TEMPLATE_NAME => Ok(TemplateKind::Default),
            REMOTE_TEMPLATE_NAME => Ok(TemplateKind::Remote),
            FULL_HASH_TEMPLATE_NAME => Ok(TemplateKind::FullHash),
            PACKAGE_DEFAULT_TEMPLATE_NAME => Ok(TemplateKind::PackageDefault),
            PACKAGE_REMOTE_TEMPLATE_NAME => Ok(TemplateKind::PackageRemote),
            PACKAGE_FULL_HASH_TEMPLATE_NAME => Ok(TemplateKind::PackageFullHash),
            MONOREPO_DEFAULT_TEMPLATE_NAME => Ok(TemplateKind::MonorepoDefault),
            MONOREPO_REMOTE_TEMPLATE_NAME => Ok(TemplateKind::MonorepoRemote),
            MONOREPO_FULL_HASH_TEMPLATE_NAME => Ok(TemplateKind::MonorepoFullHash),
            GITHUB_TEMPLATE_NAME => Ok(TemplateKind::Github),
            path => {
                let path = PathBuf::from(path);
                if !path.exists() {
                    return Err(ChangelogError::TemplateNotFound(path));
                }

                Ok(TemplateKind::Custom(path))
            }
        }
    }

    pub(crate) fn get(&self) -> Result<Vec<u8>, io::Error> {
        match self {
            TemplateKind::Default => Ok(DEFAULT_TEMPLATE.to_vec()),
            TemplateKind::Remote => Ok(REMOTE_TEMPLATE.to_vec()),
            TemplateKind::FullHash => Ok(FULL_HASH_TEMPLATE.to_vec()),
            TemplateKind::PackageDefault => Ok(PACKAGE_DEFAULT_TEMPLATE.to_vec()),
            TemplateKind::PackageRemote => Ok(PACKAGE_REMOTE_TEMPLATE.to_vec()),
            TemplateKind::PackageFullHash => Ok(PACKAGE_FULL_HASH_TEMPLATE.to_vec()),
            TemplateKind::MonorepoDefault => Ok(MONOREPO_DEFAULT_TEMPLATE.to_vec()),
            TemplateKind::MonorepoRemote => Ok(MONOREPO_REMOTE_TEMPLATE.to_vec()),
            TemplateKind::MonorepoFullHash => Ok(MONOREPO_FULL_HASH_TEMPLATE.to_vec()),
            TemplateKind::Github => Ok(GITHUB_TEMPLATE.to_vec()),
            TemplateKind::Custom(path) => std::fs::read(path),
        }
    }

    pub(crate) const fn name(&self) -> &'static str {
        match self {
            TemplateKind::Default => DEFAULT_TEMPLATE_NAME,
            TemplateKind::Remote => REMOTE_TEMPLATE_NAME,
            TemplateKind::FullHash => FULL_HASH_TEMPLATE_NAME,
            TemplateKind::PackageDefault => PACKAGE_DEFAULT_TEMPLATE_NAME,
            TemplateKind::PackageRemote => PACKAGE_REMOTE_TEMPLATE_NAME,
            TemplateKind::PackageFullHash => PACKAGE_FULL_HASH_TEMPLATE_NAME,
            TemplateKind::MonorepoDefault => MONOREPO_DEFAULT_TEMPLATE_NAME,
            TemplateKind::MonorepoRemote => MONOREPO_REMOTE_TEMPLATE_NAME,
            TemplateKind::MonorepoFullHash => MONOREPO_FULL_HASH_TEMPLATE_NAME,
            TemplateKind::Github => GITHUB_TEMPLATE_NAME,
            TemplateKind::Custom(_) => "custom_template",
        }
    }
}
