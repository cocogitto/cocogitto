use crate::git::commit::Commit;

pub struct Changelog {
  pub from: String,
  pub to: String,
  pub date: String,
  pub commits: Vec<Commit>,
}

impl Changelog {
  pub fn markdown_header() -> String {
    let mut out = 
    #"# Changelog

    All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.
    "#;
  }
  pub fn tag_diff_to_markdown(&mut self) -> String {
    let mut out = String::new(); 
    out.push_str(&format!("## {}..{} - {}\n\n", self.from, self.to, self.date));

    out.push_str("### Features\n\n");
    self.commits.drain_filter(|commit|commit.commit_type == "feat")
    .for_each(|commit|out.push_str(&commit.description));
    
    out.push_str("### Bug Fixes\n\n");
    self.commits.drain_filter(|commit|commit.commit_type == "fix")
    .for_each(|commit|out.push_str(&commit.description));

    out.push_str("### Performance Improvements\n\n");
    self.commits.drain_filter(|commit|commit.commit_type == "perf")
    .for_each(|commit|out.push_str(&commit.description));

    out.push_str("### Revert\n\n");
    self.commits.drain_filter(|commit|commit.commit_type == "revert")
    .for_each(|commit|out.push_str(&commit.description));

    out.push_str("### Misellaneous Chores\n\n");
    self.commits.drain_filter(|commit|commit.commit_type == "chore")
    .for_each(|commit|out.push_str(&commit.description));

    out.push_str("### Documentation\n\n");
    self.commits.drain_filter(|commit|commit.commit_type == "docs")
    .for_each(|commit|out.push_str(&commit.description));

    out.push_str("### Style\n\n");
    self.commits.drain_filter(|commit|commit.commit_type == "style")
    .for_each(|commit|out.push_str(&commit.description));

    out.push_str("### Refactoring\n\n");
    self.commits.drain_filter(|commit|commit.commit_type == "refactor")
    .for_each(|commit|out.push_str(&commit.description));

    out.push_str("### Tests\n\n");
    self.commits.drain_filter(|commit|commit.commit_type == "test")
    .for_each(|commit|out.push_str(&commit.description));
    
    out.push_str("### Build System\n\n");
    self.commits.drain_filter(|commit|commit.commit_type == "build")
    .for_each(|commit|out.push_str(&commit.description));

    out.push_str("### Continuous Integration\n\n");
    self.commits.drain_filter(|commit|commit.commit_type == "ci")
    .for_each(|commit|out.push_str(&commit.description));

    // TODO: add commit type from config

    out
  }
}