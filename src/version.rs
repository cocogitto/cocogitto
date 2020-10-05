use crate::commit::{Commit, CommitType};
use crate::repository::Repository;
use anyhow::Result;
use colored::*;
use git2::Commit as Git2Commit;
use semver::{Identifier, Version};

pub enum VersionIncrement {
    Major,
    Minor,
    Patch,
    Auto,
    Manual(String),
}

impl VersionIncrement {
    pub(crate) fn bump(&self, current_version: &Version) -> Result<Version> {
        match self {
            VersionIncrement::Manual(version) => {
                Version::parse(&version).map_err(|err| anyhow!(err))
            }
            VersionIncrement::Auto => self.get_auto_version(current_version),
            VersionIncrement::Major => {
                let mut next = current_version.clone();
                next.increment_major();
                Ok(next)
            }
            VersionIncrement::Patch => {
                let mut next = current_version.clone();
                next.increment_patch();
                Ok(next)
            }
            VersionIncrement::Minor => {
                let mut next = current_version.clone();
                next.increment_minor();
                Ok(next)
            }
        }
    }

    fn get_auto_version(&self, current_version: &Version) -> Result<Version> {
        let mut next_version = current_version.clone();
        let repository = Repository::open(".")?;
        let changelog_start_oid = repository
            .get_latest_tag_oid()
            .unwrap_or_else(|_| repository.get_first_commit().unwrap());

        let head = repository.get_head_commit_oid()?;

        let commits = repository.get_commit_range(changelog_start_oid, head)?;

        let commits: Vec<&Git2Commit> = commits
            .iter()
            .filter(|commit| !commit.message().unwrap_or("").starts_with("Merge "))
            .collect();

        for commit in commits {
            let commit = Commit::from_git_commit(&commit);

            // TODO: prompt for continue on err
            if let Err(err) = commit {
                eprintln!("{}", err);
            } else {
                let commit = commit.unwrap();
                match (
                    &commit.message.commit_type,
                    commit.message.is_breaking_change,
                ) {
                    (CommitType::Feature, false) => {
                        next_version.increment_minor();
                        println!(
                            "Found feature commit {}, bumping to {}",
                            commit.shorthand().blue(),
                            next_version.to_string().green()
                        )
                    }
                    (CommitType::BugFix, false) => {
                        next_version.increment_patch();
                        println!(
                            "Found bug fix commit {}, bumping to {}",
                            commit.shorthand().blue(),
                            next_version.to_string().green()
                        )
                    }
                    (commit_type, true) => {
                        next_version.increment_major();
                        println!(
                            "Found {} commit {} with type : {}",
                            "BREAKING CHANGE".red(),
                            commit.shorthand().blue(),
                            commit_type.get_key_str().yellow()
                        )
                    }
                    (_, false) => println!(
                        "Skipping irrelevant commit {} with type : {}",
                        commit.shorthand().blue(),
                        commit.message.commit_type.get_key_str().yellow()
                    ),
                }
            }
        }

        Ok(next_version)
    }
}

pub fn parse_pre_release(string: &str) -> Result<Vec<Identifier>> {
    if !string
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-')
    {
        return Err(anyhow!("Pre-release string must be a dot-separated list of identifiers comprised of ASCII alphanumerics and hyphens [0-9A-Za-z-]"));
    }

    if string.starts_with('.') || string.contains("..") || string.ends_with('.') {
        return Err(anyhow!(
            "Dot-separated identifiers in the pre-release string must not be empty"
        ));
    }

    let idents = string
        .split('.')
        .map(|s| match s.parse::<u64>() {
            Ok(n) => Identifier::Numeric(n),
            Err(_) => Identifier::AlphaNumeric(s.to_string()),
        })
        .collect();

    Ok(idents)
}

#[cfg(test)]
mod test {
    use crate::version::{parse_pre_release, VersionIncrement};
    use anyhow::Result;
    use semver::{Identifier, Version};

    // Auto version tests resides in test/ dir since it rely on git log
    // To generate the version

    #[test]
    fn major_bump() -> Result<()> {
        let version = VersionIncrement::Major.bump(&Version::new(1, 0, 0))?;
        assert_eq!(version, Version::new(2, 0, 0));
        Ok(())
    }

    #[test]
    fn minor_bump() -> Result<()> {
        let version = VersionIncrement::Minor.bump(&Version::new(1, 0, 0))?;
        assert_eq!(version, Version::new(1, 1, 0));
        Ok(())
    }

    #[test]
    fn patch_bump() -> Result<()> {
        let version = VersionIncrement::Patch.bump(&Version::new(1, 0, 0))?;
        assert_eq!(version, Version::new(1, 0, 1));
        Ok(())
    }

    #[test]
    fn parse_pre_release_valid() -> Result<()> {
        let idents = parse_pre_release("alpha.0-dev.1")?;
        assert_eq!(
            &idents,
            &[
                Identifier::AlphaNumeric("alpha".into()),
                Identifier::AlphaNumeric("0-dev".into()),
                Identifier::Numeric(1),
            ]
        );
        Ok(())
    }

    #[test]
    fn parse_pre_release_non_ascii() {
        assert!(parse_pre_release("РАСТ").is_err());
    }

    #[test]
    fn parse_pre_release_illegal_ascii() {
        assert!(parse_pre_release("alpha$5").is_err());
    }

    #[test]
    fn parse_pre_release_empty_ident() {
        assert!(parse_pre_release(".alpha.5").is_err());
        assert!(parse_pre_release("alpha..5").is_err());
        assert!(parse_pre_release("alpha.5.").is_err());
    }
}
