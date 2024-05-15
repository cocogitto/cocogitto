use crate::conventional::commit::Commit;

use conventional_commit_parser::commit::CommitType;
use git2::Commit as Git2Commit;

#[derive(Eq, PartialEq)]
pub enum CommitFilter {
    Type(CommitType),
    Scope(String),
    Author(String),
    BreakingChange,
    NoError,
}

pub struct CommitFilters(pub Vec<CommitFilter>);

impl CommitFilters {
    pub(crate) fn no_error(&self) -> bool {
        !self.0.contains(&CommitFilter::NoError)
    }

    pub(crate) fn filter_git2_commit(&self, commit: &Git2Commit) -> bool {
        // Author filters
        let authors: Vec<&String> = self
            .0
            .iter()
            .filter_map(|filter| match filter {
                CommitFilter::Author(author) => Some(author),
                _ => None,
            })
            .collect();

        let filter_authors = if authors.is_empty() {
            true
        } else {
            authors
                .iter()
                .any(|author| Some(author.as_str()) == commit.author().name())
        };

        filter_authors
    }

    pub(crate) fn filters(&self, commit: &Commit) -> bool {
        // Commit type filters
        let types: Vec<&CommitType> = self
            .0
            .iter()
            .filter_map(|filter| match filter {
                CommitFilter::Type(commit_type) => Some(commit_type),
                _ => None,
            })
            .collect();

        let filter_type = if types.is_empty() {
            true
        } else {
            types
                .iter()
                .any(|commit_type| **commit_type == commit.conventional.commit_type)
        };

        // Scope filters
        let scopes: Vec<&String> = self
            .0
            .iter()
            .filter_map(|filter| match filter {
                CommitFilter::Scope(scope) => Some(scope),
                _ => None,
            })
            .collect();

        let filter_scopes = if scopes.is_empty() {
            true
        } else {
            scopes
                .iter()
                .any(|&scope| Some(scope) == commit.conventional.scope.as_ref())
        };

        // Breaking changes filters
        let filter_breaking_changes = if self.0.contains(&CommitFilter::BreakingChange) {
            commit.conventional.is_breaking_change
        } else {
            true
        };

        filter_type && filter_scopes && filter_breaking_changes
    }
}
