use crate::git::commit::Commit;
use crate::git::changelog::CommitType::*;

pub struct Changelog {
    pub from: String,
    pub to: String,
    pub date: String,
    pub commits: Vec<Commit>,
}

const HEADER: &str = r#"# Changelog

    All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.
    "#;


enum CommitType<'a> {
    Feature,
    BugFix,
    Chore,
    Revert,
    Performances,
    Documentation,
    Style,
    Refactoring,
    Test,
    Build,
    Ci,
    Custom(&'a str, &'a str),
}

impl CommitType<'_> {
    fn get_key(&self) -> &str {
        match self {
            Feature => "feat",
            BugFix => "fix",
            Chore => "chore",
            Revert => "revert",
            Performances => "perf",
            Documentation => "docs",
            Style => "style",
            Refactoring => "refactor",
            Test => "test",
            Build => "build",
            Ci => "ci",
            Custom(key, _) => key
        }
    }

    fn get_markdown_title(&self) -> &str {
        match self {
            Feature => "Feature",
            BugFix => "Bug Fixes",
            Chore => "Misellaneous Chores",
            Revert => "Revert",
            Performances => "Performance Improvements",
            Documentation => "Documentation",
            Style => "Style",
            Refactoring => "Refactoring",
            Test => "Tests",
            Build => "Build System",
            Ci => "Continuous Integration",
            Custom(_, value) => value,
        }
    }
}

impl Changelog {
    pub fn tag_diff_to_markdown(&mut self) -> String {
        let mut out = String::new();
        out.push_str(&format!("## {}..{} - {}\n\n", self.from, self.to, self.date));

        let mut add_commit_section = |commit_type: CommitType| {
            let commits: Vec<Commit> = self.commits.drain_filter(|commit| commit.commit_type == commit_type.get_key()).collect();

            if !commits.is_empty() {
                out.push_str(&format!("### {}\n\n", commit_type.get_markdown_title()));
                commits.iter().for_each(|commit| out.push_str(&commit.description));
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
}