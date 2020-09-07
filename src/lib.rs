#![feature(drain_filter)]
#[macro_use]
extern crate anyhow;

mod git;
use crate::git::changelog::Changelog;
use git2::{Oid, Repository};
use git::commit::Commit;
use chrono::Utc;

pub fn get_changelog(from: &str, to: &str) -> anyhow::Result<String> {
    let from_oid = Oid::from_str(from)?;
    let to_oid = Oid::from_str(to)?;
    let commits = get_changelog_from_oid_range(from_oid, to_oid)?;

    let date  = Utc::now().naive_utc().date().to_string();

    let mut changelog = Changelog {
        from: from.to_owned(),
        to: to.to_owned(),
        date,
        commits,
    };
    
    Ok(changelog.tag_diff_to_markdown())
}

pub fn get_changelog_from_tags(from: &str, to: &str) -> anyhow::Result<String> {
    let from_oid = resolve_lightweight_tags_oid(from)?;
    let to_oid = resolve_lightweight_tags_oid(to)?;
    let commits = get_changelog_from_oid_range(from_oid, to_oid)?;
    let date  = Utc::now().naive_utc().date().to_string();

    let mut changelog = Changelog {
        from: from.to_owned(),
        to: to.to_owned(),
        date,
        commits,
    };

    Ok(changelog.tag_diff_to_markdown())
}

fn get_changelog_from_oid_range<'a>(from: Oid, to: Oid) -> anyhow::Result<Vec<Commit<'a>>> {
    let repo = Repository::open("./")?;
   
    // Ensure commit exists
    repo.find_commit(from)?;
    repo.find_commit(to)?;

    let mut revwalk = repo.revwalk()?;
    revwalk.push(to)?;
    revwalk.push(from)?; 
    
    let mut commits = vec![]; 
    
    for oid in revwalk {
        let oid = oid?;

        if oid == from {
            break
        }

        let commit = repo.find_commit(oid)?;
        commits.push(Commit::from_git_commit(commit));
    }

    Ok(commits)
}

fn resolve_lightweight_tags_oid(tag: &str) -> anyhow::Result<Oid> {
    let repo = Repository::open("./")?;

    repo.resolve_reference_from_short_name(tag)
    .map(|reference | reference.target().unwrap())
    .map_err(|err | anyhow!("Cannot resolve tag {} : {}", tag, err.message()))
}

#[cfg(test)]
mod test {
    #[test]
    fn should_open_repo() {}
}
