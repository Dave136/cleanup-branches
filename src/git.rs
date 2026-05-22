use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use git2::{BranchType, FetchOptions, Oid, Repository};

use crate::colour::{Colour, paint};

pub struct MergedBranch {
    pub name: String,
}

pub fn open_repo(path: &PathBuf) -> Result<Repository> {
    Repository::open(path).with_context(|| {
        format!(
            "'{}' is not a Git repository or does not exist",
            path.display()
        )
    })
}

pub fn resolve_base_oid(repo: &Repository, base_branch: &str) -> Result<Oid> {
    let local_ref = format!("refs/heads/{base_branch}");
    let remote_ref = format!("refs/remotes/origin/{base_branch}");

    let exists_local = repo.find_reference(&local_ref).is_ok();
    let exists_remote = repo.find_reference(&remote_ref).is_ok();

    if !exists_local && !exists_remote {
        bail!("Branch '{base_branch}' does not exist locally or remotely");
    }

    println!(
        "{}",
        paint(
            Colour::Blue,
            &format!("Updating information for branch '{base_branch}'...")
        )
    );
    fetch_base(repo, base_branch);

    repo.find_reference(&local_ref)
        .or_else(|_| repo.find_reference(&remote_ref))
        .with_context(|| format!("Branch '{base_branch}' not found after fetch"))?
        .peel_to_commit()
        .with_context(|| format!("Could not resolve commit for '{base_branch}'"))
        .map(|c| c.id())
}

fn fetch_base(repo: &Repository, base_branch: &str) {
    let Ok(mut remote) = repo.find_remote("origin") else {
        return;
    };
    let refspec = format!("{base_branch}:{base_branch}");
    let mut opts = FetchOptions::new();
    let _ = remote.fetch(&[refspec.as_str()], Some(&mut opts), None);
}

pub fn current_branch_name(repo: &Repository) -> Option<String> {
    let head = repo.head().ok()?;
    if head.is_branch() {
        head.shorthand().map(|s| s.to_owned())
    } else {
        None
    }
}

pub fn find_merged_branches(
    repo: &Repository,
    base_oid: Oid,
    base_branch: &str,
) -> Result<Vec<MergedBranch>> {
    println!(
        "{}",
        paint(
            Colour::Blue,
            &format!("Searching for branches merged into '{base_branch}'...")
        )
    );

    let branches = repo
        .branches(Some(BranchType::Local))
        .context("Could not list branches")?
        .filter_map(|entry| {
            let (branch, _) = entry.ok()?;
            let name = branch.name().ok()??.to_owned();

            if name == base_branch {
                return None;
            }

            let branch_oid = branch.get().peel_to_commit().ok()?.id();
            let (ahead, _) = repo.graph_ahead_behind(branch_oid, base_oid).ok()?;

            (ahead == 0).then_some(MergedBranch { name })
        })
        .collect();

    Ok(branches)
}

pub fn delete_branches(repo: &Repository, branches: &[&MergedBranch]) -> (usize, usize) {
    let mut deleted = 0usize;
    let mut failed = 0usize;

    println!("\n{}", paint(Colour::Blue, "Deleting branches..."));

    for b in branches {
        match repo.find_branch(&b.name, BranchType::Local) {
            Ok(mut branch) => match branch.delete() {
                Ok(_) => {
                    println!(
                        "  {}",
                        paint(Colour::Green, &format!("✓ Deleted: {}", b.name))
                    );
                    deleted += 1;
                }
                Err(e) => {
                    println!(
                        "  {}",
                        paint(Colour::Red, &format!("✗ Error deleting '{}': {e}", b.name))
                    );
                    failed += 1;
                }
            },
            Err(e) => {
                println!(
                    "  {}",
                    paint(Colour::Red, &format!("✗ Error finding '{}': {e}", b.name))
                );
                failed += 1;
            }
        }
    }

    (deleted, failed)
}
