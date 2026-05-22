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

### Binary release (Linux x86_64/aarch64)

Use the automated install script:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/Dave136/cleanup-branches/main/install.sh | sh
```

The default installation directory is `~/.local/bin`. To install elsewhere, set the `DIR` variable:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/Dave136/cleanup-branches/main/install.sh | DIR=/usr/local/bin sh
```

Or download and extract a release manually from the [releases page](https://github.com/Dave136/cleanup-branches/releases).

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

### Optional: Alias and system-wide installation

**Create a shorter alias (`cb`):**

Add to your shell configuration (`.bashrc`, `.zshrc`, etc.):

```sh
alias cb='cleanup-branches'
```

Or install via `cargo` and create a system-wide symlink:

```sh
# Install with cargo
cargo install --path .

# Create symlink in /usr/bin (requires sudo)
sudo ln -s ~/.cargo/bin/cleanup-branches /usr/bin/cleanup-branches

# Optional: also symlink the short alias
sudo ln -s ~/.cargo/bin/cleanup-branches /usr/bin/cb
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

## License

[MIT](./LICENSE)