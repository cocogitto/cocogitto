use crate::conventional::changelog::context::{RemoteContext, ToContext};
use crate::conventional::changelog::error::ChangelogError;
use crate::conventional::changelog::release::Release;
use crate::SETTINGS;

use super::filters;
use std::io;
use std::path::PathBuf;
use tera::{Context, Tera};

const DEFAULT_TEMPLATE: &[u8] = include_bytes!("template/simple.tera");
const DEFAULT_TEMPLATE_NAME: &str = "default";
const REMOTE_TEMPLATE: &[u8] = include_bytes!("template/remote.tera");
const REMOTE_TEMPLATE_NAME: &str = "remote";
const FULL_HASH_TEMPLATE: &[u8] = include_bytes!("template/full_hash.tera");
const FULL_HASH_TEMPLATE_NAME: &str = "full_hash";

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

pub const MACROS_TEMPLATE: &[u8] = include_bytes!("template/macro/macros.tera");
pub const MACROS_TEMPLATE_NAME: &str = "macros";

#[derive(Debug)]
pub struct Template {
    pub context: Context,
    pub kind: TemplateKind,
    pub remote_context: Option<RemoteContext>,
}

impl Template {
    pub fn from_arg(
        value: &str,
        remote_context: Option<RemoteContext>,
        unified: bool,
    ) -> Result<Self, ChangelogError> {
        let kind = TemplateKind::from_arg(value, unified)?;
        let mut context = Context::default();
        if let Some(remote) = &remote_context {
            context.extend(remote.to_context());
        }

        Ok(Template {
            context,
            kind,
            remote_context,
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
            context: Context::default(),
            remote_context: None,
        }
    }

    pub fn render(&mut self, mut version: Release) -> Result<String, ChangelogError> {
        let tera = self.init_tera()?;
        let mut release = self.render_release(&mut version, &tera)?;
        let mut version = version;
        while let Some(mut previous) = version.previous.map(|v| *v) {
            release.push_str("\n- - -\n\n");
            release.push_str(self.render_release(&mut previous, &tera)?.as_str());
            version = previous;
        }

        Ok(release)
    }

    fn init_tera(&self) -> Result<Tera, ChangelogError> {
        let mut tera = Tera::default();
        let content = self.kind.get()?;
        let content = String::from_utf8_lossy(content.as_slice());
        tera.add_raw_template(
            MACROS_TEMPLATE_NAME,
            String::from_utf8_lossy(MACROS_TEMPLATE).as_ref(),
        )?;
        tera.add_raw_template(self.kind.name(), content.as_ref())?;
        tera.register_filter("upper_first", filters::upper_first_filter);
        tera.register_filter("unscoped", filters::unscoped);
        tera.register_filter("group_by_type", filters::group_by_type);
        tera.check_macro_files()?;

        Ok(tera)
    }

    fn render_release(
        &mut self,
        version: &mut Release,
        tera: &Tera,
    ) -> Result<String, ChangelogError> {
        self.context.extend(version.to_context());
        tera.render(self.kind.name(), &self.context)
            .map_err(Into::into)
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
    fn from_arg(value: &str, unified: bool) -> Result<Self, ChangelogError> {
        match value {
            DEFAULT_TEMPLATE_NAME if !unified => Ok(TemplateKind::Default),
            DEFAULT_TEMPLATE_NAME | UNIFIED_DEFAULT_TEMPLATE_NAME => {
                Ok(TemplateKind::UnifiedDefault)
            }

            REMOTE_TEMPLATE_NAME if !unified => Ok(TemplateKind::Remote),
            REMOTE_TEMPLATE_NAME | UNIFIED_REMOTE_TEMPLATE_NAME => Ok(TemplateKind::UnifiedRemote),

            FULL_HASH_TEMPLATE_NAME if !unified => Ok(TemplateKind::FullHash),
            FULL_HASH_TEMPLATE_NAME | UNIFIED_FULL_HASH_TEMPLATE_NAME => {
                Ok(TemplateKind::UnifiedFullHash)
            }

            PACKAGE_DEFAULT_TEMPLATE_NAME => Ok(TemplateKind::PackageDefault),
            PACKAGE_REMOTE_TEMPLATE_NAME => Ok(TemplateKind::PackageRemote),
            PACKAGE_FULL_HASH_TEMPLATE_NAME => Ok(TemplateKind::PackageFullHash),

            MONOREPO_DEFAULT_TEMPLATE_NAME => Ok(TemplateKind::MonorepoDefault),
            MONOREPO_REMOTE_TEMPLATE_NAME => Ok(TemplateKind::MonorepoRemote),
            MONOREPO_FULL_HASH_TEMPLATE_NAME => Ok(TemplateKind::MonorepoFullHash),
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
