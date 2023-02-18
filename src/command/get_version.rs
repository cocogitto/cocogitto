use anyhow::bail;
use anyhow::Result;
use log::warn;
use semver::Version;

use crate::git::error::TagError;
use crate::CocoGitto;

impl CocoGitto {
    pub fn get_latest_version(
        &self,
        fallback: Option<String>,
        package: Option<String>,
    ) -> Result<()> {
        let fallback = match fallback {
            Some(input) => match Version::parse(&input) {
                Ok(version) => Some(version),
                Err(err) => {
                    warn!("Invalid fallback: {}", input);
                    bail!("{}", err)
                }
            },
            None => None,
        };

        let current_tag = match package {
            Some(pkg) => self.repository.get_latest_package_tag(&pkg),
            None => self.repository.get_latest_tag(),
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
