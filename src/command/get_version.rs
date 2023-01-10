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
        fail_on_fallback: bool,
    ) -> Result<()> {
        let fallback = match fallback {
            Some(input) => match Version::parse(input.as_ref()) {
                Ok(version) => version,
                Err(err) => {
                    warn!("Invalid fallback: {}", input);
                    bail!("{}", err)
                }
            },
            None => Version::new(0, 0, 0),
        };

        let current_tag = self.repository.get_latest_tag();
        let current_version = match current_tag {
            Ok(ref tag) => &tag.version,
            Err(ref err) if err == &TagError::NoTag => {
                if fail_on_fallback {
                    bail!("No version given")
                } else {
                    warn!(
                        "Failed to get current version, falling back to {}",
                        &fallback
                    );
                    &fallback
                }
            }
            Err(ref err) => bail!("{}", err),
        };

        warn!("Current version:");
        println!("{}", *current_version);
        Ok(())
    }
}
