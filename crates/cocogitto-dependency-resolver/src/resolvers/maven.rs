use crate::resolvers::DependencyResolver;
use serde::Deserialize;
use std::fs::File;
use std::path::Path;

pub(crate) struct MavenResolver;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
struct Project {
    group_id: Option<String>,
    artifact_id: String,
    version: Option<String>,
    dependencies: Option<Dependencies>,
    #[serde(default)]
    modules: Vec<Module>,
}

#[derive(Debug, Deserialize)]
struct Module {
    module: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Dependencies {
    dependency: Vec<Dependency>,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
#[serde(rename_all = "camelCase")]
struct Dependency {
    group_id: String,
    artifact_id: String,
    version: Option<String>,
}

impl DependencyResolver for MavenResolver {
    fn get_dependencies(&self, path: &Path) -> Vec<(String, Vec<String>)> {
        let file = File::open(path).unwrap();
        let root: Project = serde_xml_rs::from_reader(file).unwrap();
        let mut root_path = path.to_path_buf();
        root_path.pop();

        let mut dependencies = vec![];
        let modules: Vec<_> = root.modules.into_iter().flat_map(|m| m.module).collect();
        for module in &modules {
            let pom = root_path.join(module).join("pom.xml");
            let pom = File::open(pom).unwrap();
            let module: Project = serde_xml_rs::from_reader(pom).unwrap();
            let deps = module
                .dependencies
                .map(|d| d.dependency)
                .unwrap_or_default()
                .into_iter()
                .filter(|d| modules.contains(&d.artifact_id))
                .map(|d| d.artifact_id)
                .collect();
            dependencies.push((module.artifact_id, deps));
        }
        dependencies
    }
}
