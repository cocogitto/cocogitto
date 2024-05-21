use anyhow::bail;
use anyhow::Result;
use cocogitto::CogCommand;
use cocogitto_git::tag::TagLookUpOptions;
use cocogitto_tag::error::TagError;
use log::warn;
use semver::Version;

pub struct CogGetVersionCommand {
    pub fallback: Option<String>,
    pub package: Option<String>,
}

impl CogCommand for CogGetVersionCommand {
    fn execute(self) -> Result<()> {
        let repository = &Self::repository()?;
        let fallback = match self.fallback {
            Some(input) => match Version::parse(&input) {
                Ok(version) => Some(version),
                Err(err) => {
                    warn!("Invalid fallback: {}", input);
                    bail!("{}", err)
                }
            },
            None => None,
        };

        let options = TagLookUpOptions::default();
        let current_tag = match self.package {
            Some(pkg) => repository.get_latest_tag(TagLookUpOptions::package(&pkg)),
            None => repository.get_latest_tag(options),
        };

        let current_version = match current_tag {
            Ok(tag) => tag.version,
            Err(TagError::NoTag) => match fallback {
                Some(input) => input,
                None => bail!("No version yet"),
            },
            Err(err) => bail!("{}", err),
        };

        warn!("Current version:");
        print!("{current_version}");
        Ok(())
    }
}
