use std::collections::HashMap;

use tera::{get_json_pointer, to_value, try_get_value, Context, Tera, Value};

use crate::conventional::changelog::release::Release;
use crate::SETTINGS;
use itertools::Itertools;

const DEFAULT_TEMPLATE: &[u8] = include_bytes!("template/simple");
const GITHUB_TEMPLATE: &[u8] = include_bytes!("template/github");

pub struct Renderer {
    tera: Tera,
}

impl Default for Renderer {
    fn default() -> Self {
        let template = String::from_utf8_lossy(DEFAULT_TEMPLATE);
        Self::new(template.as_ref()).unwrap()
    }
}

impl Renderer {
    pub fn github() -> Self {
        let template = String::from_utf8_lossy(GITHUB_TEMPLATE);
        Self::new(template.as_ref()).unwrap()
    }

    pub fn new(template: &str) -> Result<Self, tera::Error> {
        let mut tera = Tera::default();
        tera.add_raw_template("default_changelog", template)?;
        tera.register_filter("upper_first", Self::upper_first_filter);
        tera.register_filter("unscoped", Self::unscoped);
        Ok(Renderer { tera })
    }

    pub(crate) fn render(&self, version: &Release) -> Result<String, tera::Error> {
        let template_context = Context::from_serialize(version)?;

        let mut context = tera::Context::new();
        let settings = &SETTINGS.changelog;

        if settings.github {
            if let (Some(owner), Some(repo)) = (&settings.owner, &settings.repository) {
                context.insert("platform", "https://github.com/");
                context.insert("owner", owner);
                context.insert(
                    "repository_url",
                    &format!("https://github.com/{}/{}", owner, repo),
                );
            }
        };

        context.extend(template_context);

        self.tera
            .render("default_changelog", &context)
            .map(|changelog| {
                changelog
                    .lines()
                    .map(|line| line.trim())
                    .filter(|line| *line != "\\")
                    .join("\n")
            })
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

    pub fn unscoped(value: &Value, args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
        let mut arr = try_get_value!("unscoped", "scope", Vec<Value>, value);
        if arr.is_empty() {
            return Ok(arr.into());
        }

        let value = args.get("value").unwrap_or(&Value::Null);

        let json_pointer = get_json_pointer("scope");

        arr = arr
            .into_iter()
            .filter(|v| {
                let val = v.pointer(&json_pointer).unwrap_or(&Value::Null);
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
