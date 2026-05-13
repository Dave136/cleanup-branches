# cleanup-branches

A fast CLI tool to delete local Git branches that have already been merged into a target branch.

## Features

- Detects merged branches using commit graph analysis (`git2`)
- Skips the current branch and the base branch automatically
- Colour-coded output for quick scanning
- `--dry-run` mode to preview changes before acting
- `--force` flag to skip confirmation
- `--ignore` list to protect specific branches

## Installation

### From source

Requires [Rust](https://rustup.rs/) (edition 2024, stable toolchain).

```sh
git clone <repo-url>
cd cleanup-branches
cargo build --release
```

The binary will be at `target/release/cleanup-branches`. Copy it anywhere on your `$PATH`:

```sh
cp target/release/cleanup-branches ~/.local/bin/
```

## Usage

```
cleanup-branches <repo-path> <base-branch> [OPTIONS]
```

### Arguments

| Argument | Description |
|---|---|
| `<repo-path>` | Path to the Git repository |
| `<base-branch>` | Branch to check merged status against |

### Options

| Option | Short | Description |
|---|---|---|
| `--force` | `-f` | Delete without asking for confirmation |
| `--dry-run` | `-d` | Show what would be deleted without making changes |
| `--ignore <LIST>` | `-i` | Comma-separated branch names to keep |
| `--help` | `-h` | Print help |

## Examples

```sh
# Preview branches that would be deleted
cleanup-branches /path/to/repo main --dry-run

# Delete merged branches, prompting for confirmation
cleanup-branches /path/to/repo main

# Delete without confirmation
cleanup-branches /path/to/repo main --force

# Keep specific branches even if merged
cleanup-branches /path/to/repo main --ignore hotfix,staging,release

# Check against a different base branch
cleanup-branches /path/to/repo develop
```

## How it works

The tool opens the repository with `git2` and fetches the latest state of the base branch from `origin`. It then iterates over all local branches and computes `graph_ahead_behind(branch, base)`. Any branch with `ahead == 0` is fully merged into the base — identical to what `git branch --merged` reports — and is a candidate for deletion.

## Dependencies

| Crate | Purpose |
|---|---|
| [`clap`](https://crates.io/crates/clap) | CLI argument parsing |
| [`git2`](https://crates.io/crates/git2) | Libgit2 bindings for repository operations |
| [`anyhow`](https://crates.io/crates/anyhow) | Ergonomic error handling |
