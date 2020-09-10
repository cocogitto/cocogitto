use anyhow::Result;

pub struct SemVer {
    major: u64,
    patch: u64,
    minor: u64,
}

impl SemVer {
    fn from_tag(tag: &str) -> Result<Self> {
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
}
