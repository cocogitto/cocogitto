use crate::resolvers::DependencyResolver;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct PackageJson {
    #[serde(default = "default_name")]
    name: String,
    #[serde(default)]
    dependencies: HashMap<String, String>,
    #[serde(default)]
    #[serde(rename = "devDependencies")]
    dev_dependencies: HashMap<String, String>,
    #[serde(default)]
    workspaces: Vec<String>,
}

fn default_name() -> String {
    "root".to_string()
}

pub(crate) struct NpmResolver;

impl DependencyResolver for NpmResolver {
    fn get_dependencies(&self, path: &Path) -> Vec<(String, Vec<String>)> {
        let mut workspace = path.to_path_buf();
        workspace.pop();
        let root = fs::read_to_string(path).unwrap();
        let root: PackageJson = serde_json::from_str(&root).unwrap();

        let packages_path: Vec<_> = root
            .workspaces
            .iter()
            .filter_map(|package| {
                fs::read_to_string(workspace.join(package).join("package.json")).ok()
            })
            .collect();

        let packages_name = packages_path
            .iter()
            .filter_map(|manifest| serde_json::from_str::<PackageJson>(manifest).ok())
            .map(|manifest| manifest.name)
            .collect::<Vec<String>>();

        let mut dependencies = vec![];
        for manifest in packages_path {
            let package_data: PackageJson = serde_json::from_str(&manifest).unwrap();
            let mut all_deps = Vec::new();
            all_deps.extend(
                package_data
                    .dependencies
                    .keys()
                    .filter(|dep| packages_name.contains(dep))
                    .cloned(),
            );
            all_deps.extend(
                package_data
                    .dev_dependencies
                    .keys()
                    .filter(|dep| packages_name.contains(dep))
                    .cloned(),
            );

            dependencies.push((package_data.name, all_deps));
        }

        dependencies
    }
}
