use crate::{settings::MonoRepoPackage, SETTINGS};

/// The target of a cocogitto execution
///
/// mainly used for bumping and changelogs
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Target {
    /// The non-monorepo target
    Standard,
    /// A single package
    Package {
        name: &'static str,
        package: &'static MonoRepoPackage,
    },
    /// The monorepo global (non-package) target
    Monorepo { manual: bool },
    /// A combination of the global target and all package targets
    ///
    /// Similar to [`Standard`](Target::Standard), but in a monorepo
    Unified { manual: bool },
}

impl Target {
    pub fn from_options(package: Option<&str>, manual: bool, unified: bool) -> Self {
        if SETTINGS.list_packages().is_empty() {
            Self::Standard
        } else if let Some(name) = package {
            Self::package(name)
        } else if unified {
            Self::Unified { manual }
        } else {
            Self::Monorepo { manual }
        }
    }

    pub fn package(name: &str) -> Self {
        // use iter() + find() instead of packages[name] to get static lifetime for name
        let (name, package) = SETTINGS
            .list_packages()
            .iter()
            .find(|(key, _)| *key == name)
            .expect("invalid package");
        Self::Package { name, package }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_helpers::git_init_no_gpg;

    use anyhow::Result;
    use sealed_test::prelude::*;
    use speculoos::assert_that;

    #[sealed_test]
    fn resolves_package_target() -> Result<()> {
        // Arrange
        git_init_no_gpg()?;
        std::fs::write("cog.toml", "monorepo.packages.pkg.path = \"pkg\"")?;

        // Act
        let target = Target::package("pkg");

        // Assert
        assert_that!(target).matches(|target| matches!(target, Target::Package { .. }));

        Ok(())
    }

    #[sealed_test]
    fn always_standard_outside_monorepo() -> Result<()> {
        // Arrange
        git_init_no_gpg()?;

        // Act
        let target_pkg = Target::from_options(Some("pkg"), false, false);
        let target_manual = Target::from_options(None, true, false);
        let target_unified = Target::from_options(None, false, true);
        let target_all = Target::from_options(Some("pkg"), true, true);

        // Assert
        assert_that!(target_pkg).is_equal_to(Target::Standard);
        assert_that!(target_manual).is_equal_to(Target::Standard);
        assert_that!(target_unified).is_equal_to(Target::Standard);
        assert_that!(target_all).is_equal_to(Target::Standard);

        Ok(())
    }

    #[sealed_test]
    fn resolve_package_option() -> Result<()> {
        // Arrange
        git_init_no_gpg()?;
        std::fs::write("cog.toml", "monorepo.packages.pkg.path = \"pkg\"")?;

        // Act
        let target = Target::from_options(Some("pkg"), false, false);

        // Assert
        assert_that!(target).matches(|t| matches!(t, Target::Package { .. }));

        Ok(())
    }

    #[sealed_test]
    fn resolve_unified_option() -> Result<()> {
        // Arrange
        git_init_no_gpg()?;
        std::fs::write("cog.toml", "monorepo.packages.pkg.path = \"pkg\"")?;

        // Act
        let target = Target::from_options(None, false, true);

        // Assert
        assert_that!(target).is_equal_to(Target::Unified { manual: false });

        Ok(())
    }

    #[sealed_test]
    fn resolve_manual_option() -> Result<()> {
        // Arrange
        git_init_no_gpg()?;
        std::fs::write("cog.toml", "monorepo.packages.pkg.path = \"pkg\"")?;

        // Act
        let target = Target::from_options(None, true, false);

        // Assert
        assert_that!(target).is_equal_to(Target::Monorepo { manual: true });

        Ok(())
    }
}
