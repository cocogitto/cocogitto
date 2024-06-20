use anyhow::anyhow;
use cocogitto_git::Repository;
use cog_command::CogCommand;
use log::info;
use std::path::Path;
use std::process::exit;

pub const CONFIG_PATH: &str = "cog.toml";

pub struct CogInitCommand<'a> {
    pub path: &'a Path,
}

impl<'a> CogCommand for CogInitCommand<'a> {
    fn execute(self) -> anyhow::Result<()> {
        if !self.path.exists() {
            std::fs::create_dir(self.path).map_err(|err| {
                anyhow!(
                    "failed to create directory `{:?}` \n\ncause: {}",
                    self.path,
                    err
                )
            })?;
        }

        let mut is_init_commit = false;
        let repository = match Self::repository() {
            Ok(repo) => {
                info!(
                    "Found git repository in {:?}, skipping initialisation",
                    self.path
                );
                repo
            }
            Err(_) => match Repository::init(self.path) {
                Ok(repo) => {
                    info!("Empty git repository initialized in {:?}", self.path);
                    is_init_commit = true;
                    repo
                }
                Err(err) => panic!("Unable to init repository on {:?}: {}", self.path, err),
            },
        };

        let settings = Self::settings(self.path)?;
        let settings_path = self.path.join(CONFIG_PATH);
        if settings_path.exists() {
            eprint!("Found {} in {:?}, Nothing to do", CONFIG_PATH, &self.path);
            exit(1);
        } else {
            let toml_string = toml::to_string(&settings)
                .map(|toml_string| {
                    format!(
                        "{}\n\n{}",
                        "#:schema https://docs.cocogitto.io/cog-schema.json", toml_string
                    )
                })
                .map_err(|err| anyhow!("failed to serialize {}\n\ncause: {}", CONFIG_PATH, err))?;

            std::fs::write(&settings_path, toml_string).map_err(|err| {
                anyhow!(
                    "failed to write file `{:?}`\n\ncause: {}",
                    settings_path,
                    err
                )
            })?;
        }

        repository.add_all()?;

        if is_init_commit {
            let sign = repository.gpg_sign();
            repository.commit("chore: initial commit", sign, false)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::CogInitCommand;
    use anyhow::Result;
    use cocogitto_test_helpers::*;
    use cog_command::CogCommand;
    use sealed_test::prelude::*;
    use speculoos::prelude::*;
    use std::path::Path;

    #[sealed_test]
    fn should_init_a_cog_repository() -> Result<()> {
        // Arrange
        // Act
        CogInitCommand {
            path: &Path::new("."),
        }
        .execute()?;

        // Assert
        assert_that!(Path::new("cog.toml")).exists();
        assert_that!(git_log_head_message()?).is_equal_to("chore: initial commit".to_string());
        Ok(())
    }

    #[sealed_test]
    fn should_skip_initialization_if_repository_exists() -> Result<()> {
        // Arrange
        git_init()?;
        git_commit("The first commit")?;

        // Act
        CogInitCommand {
            path: &Path::new("."),
        }
        .execute()?;

        // Assert
        assert_that!(Path::new("cog.toml")).exists();
        assert_that!(git_log_head_message()?).is_equal_to("The first commit\n".to_string());
        if cfg!(target_os = "macos") {
            assert_that!(git_status()?)
                .contains("On branch master\nChanges to be committed:\n\tnew file:   cog.toml\n");
        } else {
            assert_that!(git_status()?).contains("git restore --staged");
        }
        Ok(())
    }
}
