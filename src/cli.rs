use std::path::PathBuf;

use std::io::{self, Write};

use clap::Parser;

use anyhow::{Context, Result};

use crate::colour::{Colour, paint};
use crate::git::{
    MergedBranch, current_branch_name, delete_branches, find_merged_branches, open_repo,
    resolve_base_oid,
};

#[derive(Parser)]
#[command(
    name = "cleanup-branches",
    about = "Deletes local branches that are already merged into the specified branch.",
    override_usage = "cleanup-branches <repo-path> <base-branch> [OPTIONS]"
)]
pub struct Args {
    /// Path to the Git repository
    pub repo_path: PathBuf,

    /// Branch to check merged status against
    pub base_branch: String,

    /// Delete without confirmation
    #[arg(short, long)]
    pub force: bool,

    /// Only show what branches would be deleted (don't delete)
    #[arg(short, long = "dry-run")]
    pub dry_run: bool,

    /// Comma-separated branches to keep (e.g. hotfix,staging)
    #[arg(short, long, value_name = "LIST", value_delimiter = ',')]
    pub ignore: Vec<String>,
}

pub fn confirm_deletion() -> Result<bool> {
    print!(
        "{} ",
        paint(
            Colour::Yellow,
            "Do you want to delete these branches? (y/N):"
        )
    );
    io::stdout().flush().context("Failed to flush stdout")?;
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .context("Failed to read input")?;
    Ok(matches!(input.trim(), "y" | "Y"))
}

pub fn run() -> Result<()> {
    let args = Args::parse();

    let repo = open_repo(&args.repo_path)?;
    let base_oid = resolve_base_oid(&repo, &args.base_branch)?;
    let current = current_branch_name(&repo);
    let merged = find_merged_branches(&repo, base_oid, &args.base_branch)?;

    if merged.is_empty() {
        println!(
            "{}",
            paint(
                Colour::Green,
                &format!(
                    "No local branches merged into '{}' to delete.",
                    args.base_branch
                )
            )
        );
        return Ok(());
    }

    println!(
        "{}",
        paint(
            Colour::Yellow,
            &format!("Found {} merged branch(es):", merged.len())
        )
    );
    println!();

    for b in &merged {
        if current.as_deref() == Some(b.name.as_str()) {
            println!(
                "  {}",
                paint(
                    Colour::Yellow,
                    &format!("* {} (current branch, will be skipped)", b.name)
                )
            );
        } else if args.ignore.contains(&b.name) {
            println!(
                "  {}",
                paint(
                    Colour::Blue,
                    &format!("* {} (ignored, will be skipped)", b.name)
                )
            );
        } else {
            println!("  {}", paint(Colour::Green, &format!("- {}", b.name)));
        }
    }
    println!();

    let to_delete: Vec<&MergedBranch> = merged
        .iter()
        .filter(|b| current.as_deref() != Some(b.name.as_str()) && !args.ignore.contains(&b.name))
        .collect();

    if to_delete.is_empty() {
        println!(
            "{}",
            paint(
                Colour::Yellow,
                "No branches to delete (all are the current branch or ignored)."
            )
        );
        return Ok(());
    }

    if args.dry_run {
        println!(
            "{}",
            paint(Colour::Blue, "Dry-run mode: No branches will be deleted.")
        );
        return Ok(());
    }

    if !args.force && !confirm_deletion()? {
        println!("{}", paint(Colour::Blue, "Operation cancelled."));
        return Ok(());
    }

    let (deleted, failed) = delete_branches(&repo, &to_delete);

    println!();
    if failed == 0 {
        println!(
            "{}",
            paint(
                Colour::Green,
                &format!("✓ Process completed. Deleted {deleted} branch(es).")
            )
        );
    } else {
        println!(
            "{}",
            paint(
                Colour::Yellow,
                &format!("⚠ Process completed with errors. Deleted: {deleted}, Failed: {failed}")
            )
        );
    }

    Ok(())
}
