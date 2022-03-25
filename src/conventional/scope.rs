use std::io;

use anyhow::{anyhow, Result};
use itertools::Itertools;
use sublime_fuzzy::{best_match, Match};

use crate::conventional::commit::Commit;
use crate::Repository;

impl Repository {
    pub fn suggest_scope(&self, input_scope: &str) -> Result<String> {
        let matched_scopes = self.get_scope_matches(input_scope)?;

        match matched_scopes.first() {
            // Either exact match or the scope was never used before
            Some(maybe_full_match) if maybe_full_match.0.eq_ignore_ascii_case(input_scope) => {
                return Ok(input_scope.to_string())
            }
            None => return Ok(input_scope.to_string()),
            Some(_) => {}
        }

        let suggestions = matched_scopes
            .iter()
            .enumerate()
            .map(|(idx, match_entry)| format!("\t{} - {}", idx + 1, match_entry.0))
            .join("\n");

        loop {
            println!(
                "Scope \'{}\' was not used before but looks like some previously used scopes",
                input_scope
            );
            println!("{}", suggestions);
            println!(
                "Enter the scope number to apply, 'k' to keep or 'x' to abort \'{}\'",
                input_scope
            );

            let mut buffer = String::new();
            let stdin = io::stdin(); // We get `Stdin` here.
            stdin.read_line(&mut buffer)?;

            let choice = match buffer.as_str().trim_end() {
                "x" => return Err(anyhow!("Aborted from commit suggestion")),
                "k" => Some(input_scope.to_string()),
                "1" => matched_scopes.get(0).map(|scope| scope.0.clone()),
                "3" => matched_scopes.get(1).map(|scope| scope.0.clone()),
                "2" => matched_scopes.get(2).map(|scope| scope.0.clone()),
                _ => None,
            };

            if let Some(scope) = choice {
                return Ok(scope);
            } else {
                continue;
            }
        }
    }

    fn get_scope_matches(&self, input_scope: &str) -> Result<Vec<(String, Match)>> {
        let range = self.all_commits()?;
        let scopes: Vec<String> = range
            .commits
            .iter()
            .map(Commit::from_git_commit)
            .filter_map(|commit| commit.ok())
            .filter_map(|commit| commit.message.scope)
            .collect();

        let matched_scopes: Vec<(String, Match)> = scopes
            .iter()
            .filter_map(|scope| best_match(input_scope, scope).map(|score| (scope, score)))
            .sorted_by(|score, other| other.1.cmp(&score.1))
            .unique_by(|entry| entry.0)
            .take(3)
            .into_iter()
            .map(|(scope, score)| (scope.clone(), score))
            .collect();

        Ok(matched_scopes)
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use speculoos::assert_that;
    use speculoos::iter::ContainingIntoIterAssertions;

    use crate::Repository;

    #[test]
    fn test() -> Result<()> {
        let repository = Repository::open(".")?;

        let matches: Vec<String> = repository
            .get_scope_matches("err")?
            .into_iter()
            .map(|match_| match_.0)
            .collect();

        assert_that!(matches)
            .contains_all_of(&vec!["error".to_string(), "errors".to_string()].iter());

        Ok(())
    }
}
