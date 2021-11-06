use anyhow::bail;
use anyhow::Result;
use std::io;
use std::path::PathBuf;

const DEFAULT_TEMPLATE: &[u8] = include_bytes!("template/simple");
const DEFAULT_TEMPLATE_NAME: &str = "default";
const REMOTE_TEMPLATE: &[u8] = include_bytes!("template/remote");
const REMOTE_TEMPLATE_NAME: &str = "remote";

#[derive(Debug, Default)]
pub struct Template {
    pub context: Option<RemoteContext>,
    pub kind: TemplateKind,
}

impl Template {
    pub fn from_arg(value: &str, context: Option<RemoteContext>) -> Result<Self> {
        let template = TemplateKind::from_arg(value)?;

        Ok(Template {
            context,
            kind: template,
        })
    }
}

#[derive(Debug)]
pub enum TemplateKind {
    Default,
    Remote,
    Custom(PathBuf),
}

impl Default for TemplateKind {
    fn default() -> Self {
        TemplateKind::Default
    }
}

impl TemplateKind {
    /// Returns either a predefined template or a custom template
    fn from_arg(value: &str) -> Result<Self> {
        match value {
            DEFAULT_TEMPLATE_NAME => Ok(TemplateKind::Default),
            REMOTE_TEMPLATE_NAME => Ok(TemplateKind::Remote),
            path => {
                let path = PathBuf::from(path);
                if !path.exists() {
                    bail!("Changelog template not found at {:?}", path);
                }

                Ok(TemplateKind::Custom(path))
            }
        }
    }

    pub(crate) fn get(&self) -> Result<Vec<u8>, io::Error> {
        match self {
            TemplateKind::Default => Ok(DEFAULT_TEMPLATE.to_vec()),
            TemplateKind::Remote => Ok(REMOTE_TEMPLATE.to_vec()),
            TemplateKind::Custom(path) => std::fs::read(path),
        }
    }

    pub(crate) const fn name(&self) -> &'static str {
        match self {
            TemplateKind::Default => DEFAULT_TEMPLATE_NAME,
            TemplateKind::Remote => REMOTE_TEMPLATE_NAME,
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

impl RemoteContext {
    pub fn new(remote: String, repository: String, owner: String) -> Self {
        Self {
            remote,
            repository,
            owner,
        }
    }

    pub(crate) fn to_tera_context(&self) -> tera::Context {
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
