use crate::commit::{Commit, CommitType};
use crate::COMMITS_METADATA;
use colored::*;
use git2::Oid;

pub struct Changelog {
    pub from: Oid,
    pub to: Oid,
    pub date: String,
    pub commits: Vec<Commit>,
}

impl Changelog {
    pub fn tag_diff_to_markdown(&mut self) -> String {
        let mut out = String::new();

        out.push_str(&Changelog::header());
        let short_to = &self.to.to_string()[0..6];
        let short_from = &self.from.to_string()[0..6];
        out.push_str(&format!(
            "## {}..{} - {}\n\n",
            short_from, short_to, self.date
        ));

        let add_commit_section = |commit_type: &CommitType| {
            let commits: Vec<Commit> = self
                .commits
                .drain_filter(|commit| &commit.message.commit_type == commit_type)
                .collect();

            let metadata = COMMITS_METADATA.get(&commit_type).unwrap();
            if !commits.is_empty() {
                out.push_str(&format!("\n### {}\n\n", metadata.changelog_title.red()));
                commits
                    .iter()
                    .for_each(|commit| out.push_str(&commit.to_markdown()));
            }
        };

        COMMITS_METADATA
            .iter()
            .map(|(commit_type, _)| commit_type)
            .for_each(add_commit_section);

        out
    }

    fn header() -> String {
        let title = "# Changelog".red();
        let link = "[conventional commits]".magenta();
        format!(
            "{}\nAll notable changes to this project will be documented in this file. \
        See {}(https://www.conventionalcommits.org/) for commit guidelines.\n\n",
            title, link
        )
    }
}
