use std::path::PathBuf;

use cocogitto_dependency_resolver::DepGraphResolver;
use speculoos::prelude::*;

#[test]
fn cocogitto_workspace() -> anyhow::Result<()> {
    let resolver = DepGraphResolver::Cargo;
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR")?;
    let cargo_manifest = PathBuf::from(crate_dir).join("Cargo.toml");
    let dependencies = resolver.topological_sort(cargo_manifest);
    let dependencies: Vec<_> = dependencies.iter().map(String::as_str).collect();

    assert_that!(dependencies).is_equal_to(vec![
        "cocogitto-test-helpers",
        "cocogitto-dependency-resolver",
        "cocogitto",
    ]);

    Ok(())
}

// ┌───────────┐     ┌───────────┐
// │ package-b │ ◀── │ package-a │
// └───────────┘     └───────────┘
// │
// │
// ▼
// ┌───────────┐
// │ package-c │ ─┐
// └───────────┘  │
// │              │
// │              │
// ▼              │
// ┌───────────┐  │
// │ package-d │  │
// └───────────┘  │
// │              │
// │              │
// ▼              │
// ┌───────────┐  │
// │ package-e │ ◀┘
// └───────────┘

#[test]
fn cargo_workspace() {
    let resolver = DepGraphResolver::Cargo;
    let dependencies = resolver.topological_sort("tests/lang/cargo_workspace/Cargo.toml");
    let dependencies: Vec<_> = dependencies.iter().map(String::as_str).collect();

    assert_that!(dependencies).is_equal_to(vec![
        "package-e",
        "package-d",
        "package-c",
        "package-b",
        "package-a",
    ])
}

#[test]
fn mvn_workspace() {
    let resolver = DepGraphResolver::Maven;
    let dependencies = resolver.topological_sort("tests/lang/maven_modules/pom.xml");
    let dependencies: Vec<_> = dependencies.iter().map(String::as_str).collect();

    assert_that!(dependencies).is_equal_to(vec![
        "package-e",
        "package-d",
        "package-c",
        "package-b",
        "package-a",
    ])
}

#[test]
fn npm_workspace() {
    let resolver = DepGraphResolver::Npm;
    let dependencies = resolver.topological_sort("tests/lang/npm_workspace/package.json");
    let dependencies: Vec<_> = dependencies.iter().map(String::as_str).collect();

    assert_that!(dependencies).is_equal_to(vec![
        "package-e",
        "package-d",
        "package-c",
        "package-b",
        "package-a",
    ])
}
