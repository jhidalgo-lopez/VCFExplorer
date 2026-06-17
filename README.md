# VCFExplorer

[![CI](https://github.com/jhidalgo-lopez/VCFExplorer/actions/workflows/rust.yml/badge.svg)](https://github.com/jhidalgo-lopez/VCFExplorer/actions/workflows/rust.yml)
[![License: GPL-3.0](https://img.shields.io/badge/License-GPL--3.0-blue.svg)](LICENSE)

A terminal-based VCF (Variant Call Format) file viewer and explorer built with Rust and [Cursive](https://crates.io/crates/cursive).

## Features

- View VCF records in a tabular format (chromosome, position, ID, quality, alleles)
- Filter records by chromosome, position range, quality, and/or genotype (ref/alt allele)
- Open raw (`.vcf`) or compressed/indexed (`.vcf.gz` + `.tbi`) VCF files
- Open multiple files simultaneously
- All active filters combine via AND logic

## Screenshot

```
┌─ VCF Viewer ─────────────────────────────────────────────────────────────────┐
│ Chromosome   Position    ID  Quality  Ref Allele            Alt Allele       │
│ ───────────────────────────────────────────────────────────────────────────  │
│ 13           32872836    .    495.23  A                     C                │
│ 13           32877888    .    896.00  A                     G                │
│ …                                                                            │
└──────────────────────────────────────────────────────────────────────────────┘
```

## Requirements

- **Rust** toolchain (1.70+)
- **htslib** system library (used via `rust-htslib` FFI bindings)

| OS               | Install command                 |
| ---------------- | ------------------------------- |
| Debian / Ubuntu  | `sudo apt install libhts-dev`   |
| Fedora           | `sudo dnf install htslib-devel` |
| macOS (Homebrew) | `brew install htslib`           |

## Installation

### From source

```bash
git clone https://github.com/jhidalgo-lopez/VCFExplorer.git
cd VCFExplorer
cargo build --release
./target/release/vcfexplorer
```

### Via cargo install

```bash
cargo install --git https://github.com/jhidalgo-lopez/VCFExplorer.git
vcfexplorer
```

## Usage

### Keybindings

| Key             | Action                                |
| --------------- | ------------------------------------- |
| `Esc`           | Focus the menu bar                    |
| `q`             | Quit the application                  |
| `←` `→` `↑` `↓` | Navigate menus and dialogs            |
| `Enter`         | Select / confirm                      |
| `Tab`           | Move between fields in filter dialogs |

### Opening files

1. Press `Esc` to focus the menu bar
2. Navigate to **File → Open...**
3. Browse the file system and select a `.vcf` or `.vcf.gz` file

### Filtering

All filters are **cumulative** (AND logic). To reset all filters, use
**Filter → Clear All**.

| Filter         | Description                                         |
| -------------- | --------------------------------------------------- |
| **Chromosome** | Exact match on chromosome name (e.g. `13`)          |
| **Position**   | Range filter by start and end position (1-based)    |
| **Quality**    | Minimum quality threshold; optional maximum         |
| **Genotype**   | Match on reference allele and/or alternative allele |

### Closing files

Use **File → Close...** to remove an opened file from the current session.

### Logs

Application logs are appended to `vcf_explorer.log` in the current working
directory.

## Running Tests

```bash
cargo test
```

Tests require the `testfiles/` directory containing:

- `1kGP-subset.vcf` — uncompressed VCF (306 records)
- `1kGP-subset.vcf.gz` + `.tbi` — compressed/indexed VCF (same data)

## License

[GPL-3.0-or-later](LICENSE)
