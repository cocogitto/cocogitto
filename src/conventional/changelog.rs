use crate::conventional::commit::Commit;
use crate::{OidOf, COMMITS_METADATA};
use anyhow::Result;
use itertools::Itertools;
use std::fmt::Write;
use std::fs;
use std::path::PathBuf;

pub(crate) struct Changelog {
    pub from: OidOf,
    pub to: OidOf,
    pub date: String,
    pub commits: Vec<Commit>,
    pub tag_name: Option<String>,
}

pub(crate) struct ChangelogWriter {
    pub(crate) changelog: Changelog,
    pub(crate) path: PathBuf,
}

impl ChangelogWriter {
    pub fn write(&mut self) -> Result<()> {
        let mut changelog_content =
            fs::read_to_string(&self.path).unwrap_or_else(|_err| Changelog::changelog_template());

        let separator_idx = changelog_content.find("- - -");

        if let Some(idx) = separator_idx {
            let markdown_changelog = self.changelog.markdown(false);
            changelog_content.insert_str(idx + 5, &markdown_changelog);
            changelog_content.insert_str(idx + 5 + markdown_changelog.len(), "\n- - -");
            fs::write(&self.path, changelog_content)?;

            Ok(())
        } else {
            Err(anyhow!(
                "Cannot find default separator '- - -' in {}",
                self.path.display()
            ))
        }
    }
}

impl Changelog {
    pub(crate) fn markdown(&mut self, colored: bool) -> String {
        let short_to = &self.to;
        let short_from = &self.from;
        let version_title = self
            .tag_name
            .clone()
            .unwrap_or_else(|| format!("{}..{}", short_from, short_to));

        let mut out = format!("\n## {} - {}\n\n", version_title, self.date);

        let grouped = self
            .commits
            .drain(..)
            .map(|commit| {
                let md = commit.to_markdown(colored);
                (commit.message.commit_type, md)
            })
            .into_group_map();

        for (commit_type, commits) in grouped {
            let meta = &COMMITS_METADATA[&commit_type];

            write!(&mut out, "\n### {}\n\n", meta.changelog_title).unwrap();
            out.extend(commits);
        }

        out
    }

    pub(crate) const fn default_header() -> &'static str {
        "# Changelog\nAll notable changes to this project will be documented in this file. \
        See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.\n\n- - -\n"
    }

    pub(crate) const fn default_footer() -> &'static str {
        "\nThis changelog was generated by [cocogitto](https://github.com/oknozor/cocogitto)."
    }

    fn changelog_template() -> String {
        [Changelog::default_header(), Changelog::default_footer()].join("")
    }
}

#[cfg(test)]
mod test {
    use crate::conventional::changelog::Changelog;
    use crate::conventional::commit::Commit;
    use crate::OidOf;
    use anyhow::Result;
    use chrono::Utc;
    use conventional_commit_parser::commit::{CommitType, ConventionalCommit};
    use git2::Oid;
    use speculoos::prelude::*;

    #[test]
    fn should_generate_changelog() -> Result<()> {
        // Arrange
        let mut ch = Changelog {
            from: OidOf::Other(Oid::from_str("5375e15770ddf8821d0c1ad393d315e243014c15")?),
            to: OidOf::Other(Oid::from_str("35085f20c5293fc8830e4e44a9bb487f98734f73")?),
            date: Utc::now().date().naive_local().to_string(),
            tag_name: None,
            commits: vec![
                Commit {
                    oid: "5375e15770ddf8821d0c1ad393d315e243014c15".to_string(),
                    message: ConventionalCommit {
                        commit_type: CommitType::Feature,
                        scope: None,
                        body: None,
                        summary: "this is a commit message".to_string(),
                        is_breaking_change: false,
                        footers: vec![],
                    },
                    author: "coco".to_string(),
                    date: Utc::now().naive_local(),
                },
                Commit {
                    oid: "5375e15770ddf8821d0c1ad393d315e243014c15".to_string(),
                    message: ConventionalCommit {
                        commit_type: CommitType::Feature,
                        scope: None,
                        body: None,
                        summary: "this is an other commit message".to_string(),
                        is_breaking_change: false,
                        footers: vec![],
                    },
                    author: "cogi".to_string(),
                    date: Utc::now().naive_local(),
                },
            ],
        };

        // Act
        let content = ch.markdown(false);

        // Assert
        println!("{}", content);
        assert_that(&content)
            .contains("[5375e1](https://github.com/oknozor/cocogitto/commit/5375e15770ddf8821d0c1ad393d315e243014c15) - this is a commit message - coco");
        assert_that!(content).contains(
            "[5375e1](https://github.com/oknozor/cocogitto/commit/5375e15770ddf8821d0c1ad393d315e243014c15) - this is an other commit message - cogi"
        );
        assert_that!(content).contains("## 5375e1..35085f -");
        assert_that!(content).contains("### Features");
        assert_that!(content).does_not_contain("### Tests");
        Ok(())
    }

    #[test]
    fn should_generate_empty_changelog() -> Result<()> {
        // Arrange
        let mut ch = Changelog {
            from: OidOf::Other(Oid::from_str("5375e15770ddf8821d0c1ad393d315e243014c15")?),
            to: OidOf::Other(Oid::from_str("35085f20c5293fc8830e4e44a9bb487f98734f73")?),
            date: Utc::now().date().naive_local().to_string(),
            commits: vec![],
            tag_name: None,
        };

        // Act
        let content = ch.markdown(false);

        // Assert
        println!("{}", content);
        assert_that!(content).contains("## 5375e1..35085f");
        assert_that!(content).does_not_contain("### Features");
        Ok(())
    }
}
