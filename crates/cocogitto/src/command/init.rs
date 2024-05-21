use anyhow::anyhow;
use cocogitto_config::{Settings, CONFIG_PATH};
use cocogitto_git::Repository;
use log::info;
use std::path::Path;
use std::process::exit;

pub fn init<S: AsRef<Path> + ?Sized>(path: &S) -> anyhow::Result<()> {
    let path = path.as_ref();

    if !path.exists() {
        std::fs::create_dir(path)
            .map_err(|err| anyhow!("failed to create directory `{:?}` \n\ncause: {}", path, err))?;
    }

    let mut is_init_commit = false;
    let repository = match Repository::open(&path) {
        Ok(repo) => {
            info!(
                "Found git repository in {:?}, skipping initialisation",
                &path
            );
            repo
        }
        Err(_) => match Repository::init(&path) {
            Ok(repo) => {
                info!("Empty git repository initialized in {:?}", &path);
                is_init_commit = true;
                repo
            }
            Err(err) => panic!("Unable to init repository on {:?}: {}", &path, err),
        },
    };

    let settings = Settings::default();
    let settings_path = path.join(CONFIG_PATH);
    if settings_path.exists() {
        eprint!("Found {} in {:?}, Nothing to do", CONFIG_PATH, &path);
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
