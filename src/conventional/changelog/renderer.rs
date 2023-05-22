use std::collections::HashMap;

use tera::{dotted_pointer, to_value, try_get_value, Context, Tera, Value};

use crate::conventional::changelog::release::Release;
use crate::conventional::changelog::template::{
    MonoRepoContext, PackageContext, RemoteContext, Template, ToContext,
};

#[derive(Debug)]
pub struct Renderer {
    tera: Tera,
    context: Context,
    template: Template,
}

impl Default for Renderer {
    fn default() -> Self {
        Self::try_new(Template::default()).expect("Failed to load renderer for default template")
    }
}

impl Renderer {
    pub fn try_new(template: Template) -> Result<Self, tera::Error> {
        let mut tera = Tera::default();
        let content = template.kind.get()?;
        let content = String::from_utf8_lossy(content.as_slice());

        tera.add_raw_template(template.kind.name(), content.as_ref())?;
        tera.register_filter("upper_first", Self::upper_first_filter);
        tera.register_filter("unscoped", Self::unscoped);

        Ok(Renderer {
            tera,
            context: Context::new(),
            template,
        })
    }

    pub(crate) fn with_package_context(mut self, context: PackageContext) -> Self {
        self.context.extend(context.to_context());
        self
    }

    pub(crate) fn with_monorepo_context(mut self, context: MonoRepoContext) -> Self {
        self.context.extend(context.to_context());
        self
    }

    pub(crate) fn render(&mut self, version: Release) -> Result<String, tera::Error> {
        let mut release = self.render_release(&version)?;
        let mut version = version;
        while let Some(previous) = version.previous.map(|v| *v) {
            release.push_str("\n- - -\n\n");
            release.push_str(self.render_release(&previous)?.as_str());
            version = previous;
        }

        Ok(release)
    }

    fn render_release(&mut self, version: &Release) -> Result<String, tera::Error> {
        let release_context = Context::from_serialize(version)?;
        self.context.extend(release_context);
        let context = self
            .template
            .remote_context
            .as_ref()
            .map(RemoteContext::to_context);

        if let Some(context) = context {
            self.context.extend(context);
        }

        self.tera.render(self.template.kind.name(), &self.context)
    }

    // From git-cliff: https://github.com/orhun/git-cliff/blob/main/git-cliff-core/src/template.rs
    fn upper_first_filter(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
        let mut s = tera::try_get_value!("upper_first_filter", "value", String, value);
        let mut c = s.chars();
        s = match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        };
        Ok(tera::to_value(&s)?)
    }

    // filter commit with no scope
    fn unscoped(value: &Value, args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
        let mut arr = try_get_value!("unscoped", "scope", Vec<Value>, value);
        if arr.is_empty() {
            return Ok(arr.into());
        }

        let value = args.get("value").unwrap_or(&Value::Null);

        arr = arr
            .into_iter()
            .filter(|v| {
                let val = dotted_pointer(v, "scope").unwrap_or(&Value::Null);
                if value.is_null() {
                    val == value
                } else {
                    !val.is_null()
                }
            })
            .collect::<Vec<_>>();

        Ok(to_value(arr).unwrap())
    }
}
