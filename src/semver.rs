use anyhow::Result;

pub struct SemVer {
    major: u64,
    patch: u64,
    minor: u64,
}

impl SemVer {
    pub(crate) fn from_tag(tag: &str) -> Result<Self> {
        let parts: Vec<&str> = tag.split('.').collect();
        let major = parts[0].parse::<u64>()?;
        let patch = parts[1].parse::<u64>()?;
        let minor = parts[2].parse::<u64>()?;

        Ok(SemVer {
            major,
            patch,
            minor,
        })
    }

    pub fn inc_major(&self) -> Self {
        SemVer {
            major: self.major + 1,
            patch: 0,
            minor: 0,
        }
    }

    pub fn inc_patch(&self) -> Self {
        SemVer {
            major: self.major,
            patch: self.patch + 1,
            minor: 0,
        }
    }

    pub fn inc_minor(&self) -> Self {
        SemVer {
            major: self.major,
            patch: self.patch,
            minor: self.minor + 1,
        }
    }
}

impl Default for SemVer {
    fn default() -> Self {
        SemVer {
            major: 0,
            patch: 0,
            minor: 0,
        }
    }
}

impl ToString for SemVer {
    fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.patch, self.minor)
    }
}
