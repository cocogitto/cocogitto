use crate::resolvers::DependencyResolver;
use std::path::Path;

pub(crate) struct CargoResolver;

impl DependencyResolver for CargoResolver {
    fn get_dependencies(&self, path: &Path) -> Vec<(String, Vec<String>)> {
        let metadata = cargo_metadata::MetadataCommand::new()
            .manifest_path(path)
            .exec()
            .unwrap();

        let cargo_packages = metadata.workspace_packages();
        let mut deps: Vec<(String, Vec<String>)> = Vec::with_capacity(cargo_packages.len());

        for p in &cargo_packages {
            let packages_depedencies: Vec<_> = p
                .dependencies
                .iter()
                .filter(|d| cargo_packages.iter().any(|p| p.name == d.name))
                .collect();

            let package_deps = packages_depedencies
                .iter()
                .map(|d| d.name.clone())
                .collect();
            deps.push((p.name.clone(), package_deps));
        }

        deps
    }
}
