//! Cocogitto Dependency Resolver
//!
//! This crate provides dependency resolution functionality for Cocogitto.
use crate::resolvers::cargo::CargoResolver;
use crate::resolvers::maven::MavenResolver;
use crate::resolvers::npm::NpmResolver;
use crate::resolvers::DependencyResolver;
use petgraph::algo::toposort;
use petgraph::graphmap::DiGraphMap;
use std::path::Path;

mod resolvers;

pub enum DepGraphResolver {
    Cargo,
    Maven,
    Npm,
}

impl DepGraphResolver {
    pub fn topological_sort(&self, path: impl AsRef<Path>) -> Vec<String> {
        let mut graph = DiGraphMap::new();
        let mut all_packages = vec![];

        let dependencies = &self.get_dependencies(path);
        for (package, deps) in dependencies {
            graph.add_node(package);
            all_packages.push(package);
            for dep in deps {
                graph.add_node(dep);
                graph.add_edge(dep, package, 1);
            }
        }

        toposort(&graph, None)
            .expect("Cycle detected! Dependencies must be acyclic.")
            .into_iter()
            .cloned()
            .collect()
    }

    fn get_dependencies(&self, path: impl AsRef<Path>) -> Vec<(String, Vec<String>)> {
        let path = path.as_ref();
        match self {
            DepGraphResolver::Cargo => CargoResolver.get_dependencies(path),
            DepGraphResolver::Maven => MavenResolver.get_dependencies(path),
            DepGraphResolver::Npm => NpmResolver.get_dependencies(path),
        }
    }
}
