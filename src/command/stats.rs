use std::collections::HashMap;

use crate::{
    conventional::{changelog::release::Release, version::Increment},
    git::{oid::OidOf, tag::Tag},
    CocoGitto,
};
use anyhow::Result;
use chrono::TimeDelta;
use conventional_commit_parser::commit::CommitType;

impl CocoGitto {
    pub fn stats(&self) -> Result<()> {
        let stats = self.build_stats()?;
        let bump_stats = stats.get_bump_from_previous_release();
        let avg_commits = stats.average_commits_per_type_per_release();
        let avg_time = stats.average_time_between_releases();
        let total_releases = stats.summaries.len();

        println!("Repository Statistics:");
        println!("----------------------");
        println!("Total Releases: {}", total_releases);

        if let Some(bump_stats) = bump_stats {
            println!("Version Bumps:");
            println!("  Major: {}", bump_stats.major);
            println!("  Minor: {}", bump_stats.minor);
            println!("  Patch: {}", bump_stats.patch);
        } else {
            println!("Version Bumps: Not enough releases for comparison");
        }

        println!("\nAverage Commits per Type per Release:");
        if avg_commits.is_empty() {
            println!("  No commit data available");
        } else {
            for (commit_type, avg) in avg_commits {
                println!("  {:?}: {:.2}", commit_type, avg);
            }
        }

        println!("\nAverage Time Between Releases:");
        if let Some(time) = avg_time {
            println!("  {} days", time.num_days());
        } else {
            println!("  Not enough releases for comparison");
        }

        Ok(())
    }

    fn build_stats(&self) -> Result<RepositoryStats> {
        let commit_range = self.repository.revwalk("..")?;
        let releases = Release::try_from(commit_range.clone())?;
        let mut release_summary = Vec::new();
        let mut previous_date = None;

        for release in releases.into_iter() {
            let mut summary = ReleaseSummary::new(release.version);
            if let Some(prev) = previous_date {
                let delta = release.date.signed_duration_since(prev);
                summary.duration_since_previous = Some(delta);
            }

            summary.commit_counts = release
                .commits
                .iter()
                .map(|c| c.commit.conventional.commit_type.clone())
                .fold(HashMap::new(), |mut acc, commit_type| {
                    *acc.entry(commit_type).or_insert(0) += 1;
                    acc
                });

            release_summary.push(summary);
            previous_date = Some(release.date);
        }

        Ok(RepositoryStats::new(release_summary))
    }
}

struct RepositoryStats {
    summaries: Vec<ReleaseSummary>,
}

#[derive(Default)]
struct IncrementStats {
    major: usize,
    minor: usize,
    patch: usize,
}

impl RepositoryStats {
    fn new(summaries: Vec<ReleaseSummary>) -> Self {
        Self { summaries }
    }

    fn get_bump_from_previous_release(&self) -> Option<IncrementStats> {
        let mut stats = IncrementStats::default();

        if self.summaries.len() < 2 {
            return None;
        }

        let mut previous: Option<&Tag> = None;
        for release in &self.summaries {
            let OidOf::Tag(tag) = &release.version else {
                continue;
            };

            if let Some(previous) = previous {
                match previous.get_increment_from(tag) {
                    Some(Increment::Major) => stats.major += 1,
                    Some(Increment::Minor) => stats.minor += 1,
                    Some(Increment::Patch) => stats.patch += 1,
                    Some(Increment::NoBump) | None => {}
                };
            }

            previous = Some(tag);
        }

        Some(stats)
    }

    fn average_commits_per_type_per_release(&self) -> HashMap<CommitType, f64> {
        let summaries = &self.summaries;

        if summaries.is_empty() {
            return HashMap::new();
        }

        // Initialize two HashMaps to track:
        // 1. type_totals: cumulative count of commits for each commit type across all releases
        // 2. type_counts: number of releases that contain each commit type
        let mut type_totals: HashMap<CommitType, usize> = HashMap::new();
        let mut type_counts: HashMap<CommitType, usize> = HashMap::new();

        for summary in summaries {
            for (commit_type, count) in &summary.commit_counts {
                *type_totals.entry(commit_type.clone()).or_insert(0) += *count;
                *type_counts.entry(commit_type.clone()).or_insert(0) += 1;
            }
        }

        type_totals
            .into_iter()
            .map(|(commit_type, total)| {
                let count = type_counts.get(&commit_type).copied().unwrap_or(0);
                if count == 0 {
                    (commit_type, 0.0)
                } else {
                    (commit_type, total as f64 / count as f64)
                }
            })
            .collect()
    }

    fn average_time_between_releases(&self) -> Option<TimeDelta> {
        let summaries = &self.summaries;
        if summaries.len() < 2 {
            return None;
        }

        let total_duration: TimeDelta = summaries
            .iter()
            .filter_map(|s| s.duration_since_previous)
            .sum();

        let count = summaries
            .iter()
            .filter(|s| s.duration_since_previous.is_some())
            .count();

        if count == 0 {
            return None;
        }

        Some(total_duration / count as i32)
    }
}

struct ReleaseSummary {
    duration_since_previous: Option<TimeDelta>,
    version: OidOf,
    commit_counts: HashMap<CommitType, usize>,
}

impl ReleaseSummary {
    fn new(version: OidOf) -> Self {
        Self {
            duration_since_previous: None,
            version,
            commit_counts: HashMap::new(),
        }
    }
}
