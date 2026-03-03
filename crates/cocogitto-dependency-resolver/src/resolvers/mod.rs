use std::path::Path;

pub(super) mod cargo;
pub(super) mod maven;
pub(super) mod npm;

pub trait DependencyResolver {
    fn get_dependencies(&self, path: &Path) -> Vec<(String, Vec<String>)>;
}
