use crate::commit::{Commit, CommitType};
use colored::*;
use git2::Oid;

pub struct Changelog<'a> {
    pub from: Oid,
    pub to: Oid,
    pub date: String,
    pub commits: Vec<Commit<'a>>,
}

impl Changelog<'_> {
    pub fn tag_diff_to_markdown(&mut self) -> String {
        let mut out = String::new();
        out.push_str(&Changelog::header());
        out.push_str(&format!("## {}..{} - {}\n\n", self.from, self.to, self.date));

        let mut add_commit_section = |commit_type: CommitType| {
            let commits: Vec<Commit> = self.commits.drain_filter(|commit| commit.commit_type == commit_type).collect();

            if !commits.is_empty() {
                out.push_str(&format!("\n### {}\n\n", commit_type.get_markdown_title().red()));
                commits.iter().for_each(|commit| out.push_str(&commit.to_markdown()));
            }
        };

        add_commit_section(CommitType::Feature);
        add_commit_section(CommitType::BugFix);
        add_commit_section(CommitType::Performances);
        add_commit_section(CommitType::Revert);
        add_commit_section(CommitType::Chore);
        add_commit_section(CommitType::Documentation);
        add_commit_section(CommitType::Style);
        add_commit_section(CommitType::Refactoring);
        add_commit_section(CommitType::Test);
        add_commit_section(CommitType::Build);
        add_commit_section(CommitType::Ci);

        // TODO: add commit type from config

        out
    }

    fn header() -> String {
        let title = "# Changelog".red();
        let link = "[conventional commits]".magenta();
        format!("{}\nAll notable changes to this project will be documented in this file. \
        See {}(https://www.conventionalcommits.org/) for commit guidelines.\n\n", title, link)
    }
}