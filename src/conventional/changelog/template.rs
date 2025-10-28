use crate::conventional::changelog::error::ChangelogError;
use crate::SETTINGS;

use serde::Serialize;

use crate::git::oid::OidOf;
use std::io;
use std::path::PathBuf;
use tera::Context;

const DEFAULT_TEMPLATE: &[u8] = include_bytes!("template/simple.tera");
const DEFAULT_TEMPLATE_NAME: &str = "default";
const REMOTE_TEMPLATE: &[u8] = include_bytes!("template/remote.tera");
const REMOTE_TEMPLATE_NAME: &str = "remote";
const FULL_HASH_TEMPLATE: &[u8] = include_bytes!("template/full_hash.tera");
const FULL_HASH_TEMPLATE_NAME: &str = "full_hash";

pub const MACROS_TEMPLATE: &[u8] = include_bytes!("template/macro/macros.tera");
pub const MACROS_TEMPLATE_NAME: &str = "macros";

const PACKAGE_DEFAULT_TEMPLATE: &[u8] = include_bytes!("template/package_simple.tera");
const PACKAGE_DEFAULT_TEMPLATE_NAME: &str = "package_default";
const PACKAGE_REMOTE_TEMPLATE: &[u8] = include_bytes!("template/package_remote.tera");
const PACKAGE_REMOTE_TEMPLATE_NAME: &str = "package_remote";
const PACKAGE_FULL_HASH_TEMPLATE: &[u8] = include_bytes!("template/package_full_hash.tera");
const PACKAGE_FULL_HASH_TEMPLATE_NAME: &str = "package_full_hash";

const MONOREPO_DEFAULT_TEMPLATE: &[u8] = include_bytes!("template/monorepo_simple.tera");
const MONOREPO_DEFAULT_TEMPLATE_NAME: &str = "monorepo_default";
const MONOREPO_REMOTE_TEMPLATE: &[u8] = include_bytes!("template/monorepo_remote.tera");
const MONOREPO_REMOTE_TEMPLATE_NAME: &str = "monorepo_remote";
const MONOREPO_FULL_HASH_TEMPLATE: &[u8] = include_bytes!("template/monorepo_full_hash.tera");
const MONOREPO_FULL_HASH_TEMPLATE_NAME: &str = "monorepo_full_hash";

const UNIFIED_DEFAULT_TEMPLATE: &[u8] = include_bytes!("template/unified_simple.tera");
const UNIFIED_DEFAULT_TEMPLATE_NAME: &str = "unified_default";
const UNIFIED_REMOTE_TEMPLATE: &[u8] = include_bytes!("template/unified_remote.tera");
const UNIFIED_REMOTE_TEMPLATE_NAME: &str = "unified_remote";
const UNIFIED_FULL_HASH_TEMPLATE: &[u8] = include_bytes!("template/unified_full_hash.tera");
const UNIFIED_FULL_HASH_TEMPLATE_NAME: &str = "unified_full_hash";

#[derive(Debug, Default)]
pub struct Template {
    pub remote_context: Option<RemoteContext>,
    pub kind: TemplateKind,
}

impl Template {
    pub fn from_arg(value: &str, context: Option<RemoteContext>) -> Result<Self, ChangelogError> {
        let template = TemplateKind::from_arg(value)?;

        Ok(Template {
            remote_context: context,
            kind: template,
        })
    }

    pub fn fallback(unified: bool) -> Self {
        let kind = if SETTINGS.packages.is_empty() {
            TemplateKind::Default
        } else if unified {
            TemplateKind::UnifiedDefault
        } else {
            TemplateKind::MonorepoDefault
        };
        Self {
            kind,
            remote_context: None,
        }
    }
}

#[derive(Debug, Default)]
pub enum TemplateKind {
    #[default]
    Default,
    FullHash,
    Remote,
    PackageDefault,
    PackageFullHash,
    PackageRemote,
    MonorepoDefault,
    MonorepoFullHash,
    MonorepoRemote,
    UnifiedDefault,
    UnifiedFullHash,
    UnifiedRemote,
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
            UNIFIED_DEFAULT_TEMPLATE_NAME => Ok(TemplateKind::UnifiedDefault),
            UNIFIED_REMOTE_TEMPLATE_NAME => Ok(TemplateKind::UnifiedRemote),
            UNIFIED_FULL_HASH_TEMPLATE_NAME => Ok(TemplateKind::UnifiedFullHash),
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
            TemplateKind::UnifiedDefault => Ok(UNIFIED_DEFAULT_TEMPLATE.to_vec()),
            TemplateKind::UnifiedRemote => Ok(UNIFIED_REMOTE_TEMPLATE.to_vec()),
            TemplateKind::UnifiedFullHash => Ok(UNIFIED_FULL_HASH_TEMPLATE.to_vec()),
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
            TemplateKind::UnifiedDefault => UNIFIED_DEFAULT_TEMPLATE_NAME,
            TemplateKind::UnifiedRemote => UNIFIED_REMOTE_TEMPLATE_NAME,
            TemplateKind::UnifiedFullHash => UNIFIED_FULL_HASH_TEMPLATE_NAME,
            TemplateKind::Custom(_) => "custom_template",
        }
    }
}

/// A wrapper to append remote repository information to template context
#[derive(Debug)]
pub struct RemoteContext {
    remote: String,
    repository: String,
    owner: String,
}

#[derive(Debug)]
pub struct MonoRepoContext<'a> {
    pub package_lock: bool,
    pub packages: Vec<PackageBumpContext<'a>>,
}

#[derive(Debug, Serialize)]
pub struct PackageBumpContext<'a> {
    pub package_name: &'a str,
    pub package_path: &'a str,
    pub version: OidOf,
    pub from: Option<OidOf>,
}

#[derive(Debug)]
pub struct PackageContext<'a> {
    pub package_name: &'a str,
}

pub(crate) trait ToContext {
    fn to_context(&self) -> Context;
}

impl ToContext for MonoRepoContext<'_> {
    fn to_context(&self) -> Context {
        let mut context = tera::Context::new();
        context.insert("package_lock", &self.package_lock);
        context.insert("packages", &self.packages);
        context
    }
}

impl ToContext for PackageContext<'_> {
    fn to_context(&self) -> Context {
        let mut context = tera::Context::new();
        context.insert("package_name", &self.package_name);
        context
    }
}

impl ToContext for RemoteContext {
    fn to_context(&self) -> Context {
        let mut context = tera::Context::new();
        context.insert("platform", &format!("https://{}", self.remote.as_str()));
        context.insert("owner", self.owner.as_str());
        context.insert(
            "repository_url",
            &format!("https://{}/{}/{}", self.remote, self.owner, self.repository),
        );

        context
    }
}

impl RemoteContext {
    pub fn try_new(
        remote: Option<String>,
        repository: Option<String>,
        owner: Option<String>,
    ) -> Option<Self> {
        match (remote, repository, owner) {
            (Some(remote), Some(repository), Some(owner)) => Some(Self {
                remote,
                repository,
                owner,
            }),
            (None, None, None) => None,
            _ => panic!("Changelog remote context should be set. Missing one of 'remote', 'repository', 'owner' in changelog configuration")
        }
    }
}
