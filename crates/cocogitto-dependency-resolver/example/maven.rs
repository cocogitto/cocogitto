use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let deps = cocogitto_dependency_resolver::DepGraphResolver::Maven.topological_sort(path);
    deps.into_iter()
        .enumerate()
        .for_each(|d| println!("{}: {:?}", d.0, d.1));
}
