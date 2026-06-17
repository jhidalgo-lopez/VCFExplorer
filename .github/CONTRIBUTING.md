# Contributing to VCFExplorer

Thanks for your interest in contributing!

## Project conventions

- This is a personal Rust learning project — code review and suggestions are
  welcome, but significant rewrites may not align with learning goals.
- Single crate, no workspace. Three modules: `main.rs`, `tui.rs`, `vcf.rs`.
- TUI framework: [Cursive](https://crates.io/crates/cursive) 0.21.x.
- VCF parsing: [rust-htslib](https://crates.io/crates/rust-htslib) 1.0.0.
- State is managed through Cursive's `set_user_data`/`user_data` (see
  `AppState` in `tui.rs`).

## Setup

```bash
# Install system dependency
sudo apt install libhts-dev   # Debian/Ubuntu
brew install htslib           # macOS

# Clone and build
git clone https://github.com/jhidalgo-lopez/VCFExplorer.git
cd VCFExplorer
cargo build
cargo test
```

## Before submitting

- Run `cargo fmt` to format your code.
- Run `cargo clippy -- -D warnings` and fix any issues.
- Run `cargo test` and ensure all 4 tests pass.
- Keep changes focused and documented in the commit message.

## Feature planning

New functionality ideas are tracked in the
[GitHub Projects board](https://github.com/jhidalgo-lopez/VCFExplorer/projects).

Some planned features for future versions:

- **v0.2**: CLI argument to open a file directly on launch
- **v0.2**: Stats panel showing record count and active filters
- **v0.2**: VCF header/metadata viewer
- **v0.2**: Export filtered records to file
- **v0.3**: Column visibility toggling
- **v0.3**: Keyboard shortcuts help overlay (`?` key)
- **v0.3**: Better error handling and fewer panics
- **v0.3**: Search within records

Feedback and feature ideas are always welcome — open an issue or discussion.
