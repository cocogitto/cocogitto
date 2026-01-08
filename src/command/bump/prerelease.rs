use anyhow::{ensure, Result};
use semver::Version;

use crate::git::tag::Tag;

/// Increments a pre-release version based on a pattern.
///
/// For example, given "alpha.2" and the pattern "alpha.*", it increments to "alpha.3".
///
/// If the pre-release version doesn't match the pattern, it starts a new sequence based on the
/// pattern.
///
/// For example, given "alpha.2" and the pattern "beta.*", it returns "beta.1".
///
/// If the next version is greater than the current pre-release expected, it starts a new sequence.
/// For example, if the current pre-release is 1.2.3-alpha.1 and the next version is 2.0.0, then the
/// next pre-release would be 2.0.0-alpha.1 (not 2.0.0-alpha.2).
///
/// - current_prerelease The latest pre-release version
/// - next The next version
/// - prerelease The pre-release pattern, for example "alpha.*"
pub(super) fn increment_prerelease(
    current_prerelease: &Option<Tag>,
    next: &Tag,
    prerelease: &str,
) -> Result<String> {
    ensure!(!prerelease.is_empty(), "Pre-release can't be empty");

    if let Some(current_prerelease) = current_prerelease {
        // Ensure the current pre-release is less than next version
        ensure!(
            current_prerelease.version < next.version,
            "Current pre-release ({}) is greater than the next version ({})",
            current_prerelease,
            next
        );
    }

    // Validate the pattern contains just one asterisk ("*")
    let pattern_parts: Vec<&str> = prerelease.split('*').collect();
    ensure!(
        pattern_parts.len() == 2,
        "Pre-release pattern ({}) is invalid. Valid patterns contain exactly one wildcard (*)",
        prerelease
    );

    let (prefix, suffix) = (pattern_parts[0], pattern_parts[1]);
    let initial_prerelease = format!("{}1{}", prefix, suffix);

    let Some(current_prerelease) = current_prerelease else {
        return Ok(initial_prerelease);
    };

    // Check if the current pre-release was expecting a different version to what the next pre-release expects
    if core_version(&current_prerelease.version) < next.version {
        return Ok(initial_prerelease);
    }

    let pre_to_increment = current_prerelease.version.pre.as_str();

    // Check if the current pre-release matches the pattern format
    if !pre_to_increment.starts_with(prefix)
        || !pre_to_increment.ends_with(suffix)
        || pre_to_increment.len() < prerelease.len()
    {
        return Ok(initial_prerelease);
    }

    // Try to extract the numeric identifier
    let num = &pre_to_increment[prefix.len()..pre_to_increment.len() - suffix.len()];
    let Ok(num) = num.parse::<u64>() else {
        return Ok(initial_prerelease);
    };

    // Reconsturct the pre-release label, incrementing the numeric identifier
    Ok(format!("{}{}{}", prefix, num + 1, suffix))
}

fn core_version(version: &Version) -> Version {
    Version::new(version.major, version.minor, version.patch)
}

#[cfg(test)]
mod test {
    use anyhow::Result;

    use crate::{command::bump::increment_prerelease, git::tag::Tag};

    #[test]
    fn increment_prerelease_invalid_empty() -> Result<()> {
        let current_prerelease = Option::None::<Tag>;
        let next = Tag::from_str("1.2.3", None)?;
        let prerelease = "";

        let result = increment_prerelease(&current_prerelease, &next, prerelease);

        assert!(result.is_err());
        assert_eq!(
            "Pre-release can't be empty",
            result.unwrap_err().to_string()
        );

        Ok(())
    }

    #[test]
    fn increment_prerelease_invalid_no_pattern() -> Result<()> {
        let current_prerelease = Option::None::<Tag>;
        let next = Tag::from_str("1.2.3", None)?;
        let prerelease = "alpha.4";

        let result = increment_prerelease(&current_prerelease, &next, prerelease);

        assert!(result.is_err());
        assert_eq!(
            "Pre-release pattern (alpha.4) is invalid. Valid patterns contain exactly one wildcard (*)",
            result.unwrap_err().to_string()
        );
        Ok(())
    }

    #[test]
    fn increment_prerelease_invalid_pattern() -> Result<()> {
        let current_prerelease = Option::None::<Tag>;
        let next = Tag::from_str("1.2.3", None)?;
        let prerelease = "alpha.**";

        let result = increment_prerelease(&current_prerelease, &next, prerelease);

        assert!(result.is_err());
        assert_eq!(
            "Pre-release pattern (alpha.**) is invalid. Valid patterns contain exactly one wildcard (*)",
            result.unwrap_err().to_string()
        );

        Ok(())
    }

    #[test]
    fn increment_prerelease_invalid_pattern_2() -> Result<()> {
        let current_prerelease = Option::None::<Tag>;
        let next = Tag::from_str("1.2.3", None)?;
        let prerelease = "**";

        let result = increment_prerelease(&current_prerelease, &next, prerelease);

        assert!(result.is_err());
        assert_eq!(
            "Pre-release pattern (**) is invalid. Valid patterns contain exactly one wildcard (*)",
            result.unwrap_err().to_string()
        );

        Ok(())
    }

    #[test]
    fn increment_prerelease_invalid_pattern_3() -> Result<()> {
        let current_prerelease = Option::None::<Tag>;
        let next = Tag::from_str("1.2.3", None)?;
        let prerelease = "***";

        let result = increment_prerelease(&current_prerelease, &next, prerelease);

        assert!(result.is_err());
        assert_eq!(
            "Pre-release pattern (***) is invalid. Valid patterns contain exactly one wildcard (*)",
            result.unwrap_err().to_string()
        );

        Ok(())
    }

    #[test]
    fn increment_prerelease_initial() -> Result<()> {
        let current_prerelease = Option::None::<Tag>;
        let next = Tag::from_str("1.2.3", None)?;
        let prerelease = "alpha.*";

        let result = increment_prerelease(&current_prerelease, &next, prerelease)?;

        assert_eq!("alpha.1", result);
        Ok(())
    }

    #[test]
    fn increment_prerelease_initial_no_match() -> Result<()> {
        let current_prerelease = Tag::from_str("1.2.3-alpha.4", None)?;
        let next = Tag::from_str("1.2.3", None)?;
        let prerelease = "beta.*";

        let result = increment_prerelease(&Some(current_prerelease), &next, prerelease)?;

        assert_eq!("beta.1", result);
        Ok(())
    }

    #[test]
    fn increment_prerelease_partial_match() -> Result<()> {
        let current_prerelease = Tag::from_str("1.2.3-alpha.preview.1", None)?;
        let next = Tag::from_str("1.2.3", None)?;
        let prerelease = "alpha.*";

        let result = increment_prerelease(&Some(current_prerelease), &next, prerelease)?;

        assert_eq!("alpha.1", result);
        Ok(())
    }

    #[test]
    fn increment_prerelease_initial_after_prerelease() -> Result<()> {
        let current_prerelease = Some(Tag::from_str("1.2.3-alpha.5", None)?);
        let next = Tag::from_str("1.3.0", None)?;
        let prerelease = "alpha.*";

        let result = increment_prerelease(&current_prerelease, &next, prerelease)?;

        assert_eq!("alpha.1", result);
        Ok(())
    }

    #[test]
    fn increment_prerelease_invalid_after_later_prerelease() -> Result<()> {
        let current_prerelease = Some(Tag::from_str("1.3.0-alpha.5", None)?);
        let next = Tag::from_str("1.2.0", None)?;
        let prerelease = "alpha.*";

        let result = increment_prerelease(&current_prerelease, &next, prerelease);

        assert!(result.is_err());
        assert_eq!(
            "Current pre-release (1.3.0-alpha.5) is greater than the next version (1.2.0)",
            result.unwrap_err().to_string()
        );

        Ok(())
    }

    #[test]
    fn increment_prerelease_invalid_after_later_prerelease_and_no_pattern() -> Result<()> {
        let current_prerelease = Some(Tag::from_str("1.3.0-alpha.5", None)?);
        let next = Tag::from_str("1.2.0", None)?;
        let prerelease = "alpha.4";

        let result = increment_prerelease(&current_prerelease, &next, prerelease);

        assert!(result.is_err());
        assert_eq!(
            "Current pre-release (1.3.0-alpha.5) is greater than the next version (1.2.0)",
            result.unwrap_err().to_string()
        );

        Ok(())
    }

    #[test]
    fn increment_prerelease_wildcard_at_start() -> Result<()> {
        let current_prerelease = Tag::from_str("1.2.3-4.alpha", None)?;
        let next = Tag::from_str("1.2.3", None)?;
        let prerelease = "*.alpha";

        let result = increment_prerelease(&Some(current_prerelease), &next, prerelease)?;

        assert_eq!("5.alpha", result);
        Ok(())
    }

    #[test]
    fn increment_prerelease_wildcard_in_middle() -> Result<()> {
        let current_prerelease = Tag::from_str("1.2.3-alpha.4.fix", None)?;
        let next = Tag::from_str("1.2.3", None)?;
        let prerelease = "alpha.*.fix";

        let result = increment_prerelease(&Some(current_prerelease), &next, prerelease)?;

        assert_eq!("alpha.5.fix", result);
        Ok(())
    }

    #[test]
    fn increment_prerelease_wildcard_only() -> Result<()> {
        let current_prerelease = Tag::from_str("1.2.3-4", None)?;
        let next = Tag::from_str("1.2.3", None)?;
        let prerelease = "*";

        let result = increment_prerelease(&Some(current_prerelease), &next, prerelease)?;

        assert_eq!("5", result);
        Ok(())
    }

    #[test]
    fn increment_prerelease_wildcard_at_end() -> Result<()> {
        let current_prerelease = Tag::from_str("1.2.3-alpha.4", None)?;
        let next = Tag::from_str("1.2.3", None)?;
        let prerelease = "alpha.*";

        let result = increment_prerelease(&Some(current_prerelease), &next, prerelease)?;

        assert_eq!("alpha.5", result);
        Ok(())
    }
}
