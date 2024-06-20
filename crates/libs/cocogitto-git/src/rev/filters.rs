use cocogitto_config::monorepo::MonoRepoPackage;
use globset::{Candidate, GlobBuilder, GlobSet, GlobSetBuilder};
use std::path::Path;

#[derive(Debug)]
pub(crate) struct PackagePathFilter {
    include: GlobSet,
    exclude: GlobSet,
}

impl PackagePathFilter {
    pub(super) fn from_package(package: &MonoRepoPackage) -> Self {
        Self::new(
            package.path.to_str().expect("valid package path"),
            &package.include,
            &package.ignore,
        )
    }

    pub(super) fn is_match<P: AsRef<Path> + ?Sized>(&self, path: &P) -> bool {
        let candidate = Candidate::new(path);
        self.include.is_match_candidate(&candidate) && !self.exclude.is_match_candidate(&candidate)
    }

    fn new(package_path: &str, include_paths: &[String], exclude_paths: &[String]) -> Self {
        let include = {
            let mut builder = GlobSetBuilder::new();
            builder.add(
                GlobBuilder::new(format!("{}/**", package_path).as_str())
                    .literal_separator(true)
                    .build()
                    .expect("glob should be valid"),
            );
            for include in include_paths {
                builder.add(
                    GlobBuilder::new(include)
                        .literal_separator(true)
                        .build()
                        .expect("glob should be valid"),
                );
            }
            builder.build().expect("valid globset")
        };
        let exclude = {
            let mut builder = GlobSetBuilder::new();
            for exclude in exclude_paths {
                builder.add(
                    GlobBuilder::new(exclude)
                        .literal_separator(true)
                        .build()
                        .expect("glob should be valid"),
                );
            }
            builder.build().expect("valid globset")
        };

        PackagePathFilter { include, exclude }
    }
}
