# VCF Explorer

A terminal-based VCF (Variant Call Format) file viewer and explorer.

## Description

VCF Explorer is a tool for viewing and filtering VCF files in the terminal.

## Features

* View VCF records in a tabular format.
* Filter records by chromosome, position, quality and/or genotype.
* Open raw or compressed (indexed) VCF files.

## Installation

1. Clone the repository:

    ```bash
    git clone https://github.com/jhidalgo-lopez/VCFExplorer.git
    ```

2. Build the project:

    ```bash
    cargo build --release
    ```

3. Run the application:

    ```bash
    ./target/release/VCFExplorer
    ```

