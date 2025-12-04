evedb (Rust)
Build the eVED (Extended Vehicle Energy Dataset) SQLite database locally from the upstream source data repositories.

# Overview
This command-line tool automates cloning the source data, extracting and transforming vehicle and signal data, and loading it into a local SQLite database. It can also optionally clean up the source repositories afterward.

## Key features
- One-command build of the eVED SQLite database
- Fast CSV handling and bulk inserts with transactions
- Progress feedback while loading signal files
- Simple, discoverable CLI

## Stack and tooling
- Language: Rust (edition 2024)
- Package manager/build: Cargo
- Async runtime: tokio
- CLI: clap
- Database: rusqlite (bundled, no external SQLite required)
- Data handling: csv, calamine (XLSX), zip, serde
- HTTP client (if/when needed): reqwest

## Requirements
- Rust toolchain (recommended via rustup). Minimum stable compatible with edition 2024.
- Git (required for the clone step).
- Internet access to fetch the source datasets:
  - https://bitbucket.org/datarepo/eved_dataset.git
  - https://github.com/gsoh/VED.git
- Optional: Docker or Podman (only for the Valhalla helper scripts in the Makefile; not required for building the DB).

## Installation
1) Install Rust and Cargo
   - curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   - Or follow https://www.rust-lang.org/tools/install
2) Clone this repository
   - git clone <this-repo-url>
   - cd evedb

## Build
- Debug build: cargo build
- Release build: cargo build --release

## Run
The binary name is evedb. The CLI offers three subcommands: build, clone, and clean.

- Typical build (clone data, build DB, then clean repos):
  `cargo run -- build`

- Build without cloning (use existing repos at repo_path):
  `cargo run -- build --no-clone`

- Build without cleaning (keep cloned repos for inspection/reuse):
  `cargo run -- build --no-clean`

- Clone only:
  `cargo run -- clone`

- Clean only (remove repositories folder):
  `cargo run -- clean`

## CLI reference
Global flags and defaults:
- --repo-path <PATH>  Path to the local folder where datasets will be cloned.
  - Default: ./data/eved/repo
- --db-path <FILE>    Path to the output SQLite database file.
  - Default: ./data/eved/evedb.db
- --verbose           Verbose logging.

### Subcommands:
- build [--no-clone] [--no-clean]
  - --no-clone  Do not clone the repositories before building.
  - --no-clean  Do not remove the repositories folder after building.
- clone
  - Clones the upstream repositories into --repo-path.
- clean
  - Removes the repositories folder at --repo-path.

## Data sources and expectations
The build process expects the following data within the cloned repositories:
- From eved_dataset (cloned to {repo_path}/eved):
  - data/eVED.zip containing CSV files for signals.
- From VED (cloned to {repo_path}/ved):
  - Data/VED_Static_Data_ICE&HEV.xlsx
  - Data/VED_Static_Data_PHEV&EV.xlsx

The builder will:
1) Create tables (vehicles, signals, trajectories) in the SQLite database at --db-path
2) Load vehicles from the two XLSX files
3) Iterate through all CSV entries in eVED.zip and load signals (with basic cleaning for NaN/semicolons)
4) Build indexes for faster queries
5) Generate trajectories from loaded data

## Environment variables
- None are required by the application code at this time.
- Git must be available on PATH for the clone step.
- Optional: Docker/Podman available on PATH to use the Valhalla Makefile helpers.

## Scripts and useful commands
Cargo
- Build (debug): `cargo build`
- Build (release): `cargo build --release`
- Run: cargo run -- <subcommand> [options]
- Format (if you use rustfmt): cargo fmt
- Lint (if you use clippy): cargo clippy

### Makefile (optional, for Valhalla convenience only; not required by evedb)
- get-map           Download a sample OSM PBF file into ./valhalla/files
- docker-run        Start a Valhalla container exposing 8002, mounting ./valhalla/files
- podman-run        Same as docker-run using Podman

## Database
- The generated SQLite database is stored at --db-path (default: ./data/eved/evedb.db)
- rusqlite is compiled with the "bundled" feature, so no external SQLite installation is needed.

# Project structure
- Cargo.toml                Project manifest (name: evedb)
- src/
  - main.rs                 Entry point (Tokio async main)
  - cli.rs                  CLI definitions (clap)
  - commands/
    - build.rs              Orchestrates build steps (clone, vehicles, signals, trajectories, clean)
    - clone.rs              Clones upstream repositories
    - clean.rs              Cleans the repositories folder
  - db/
    - evedb.rs              SQLite schema and load routines (tables, inserts, indexes)
    - api.rs                Minimal DB wrapper (connection helper)
  - etl/
    - extract/
      - vehicles.rs         XLSX reading and vehicle extraction
      - signals.rs          Reads CSVs from eVED.zip and inserts signals
  - models/
    - vehicle.rs            Vehicle model
    - signal.rs             CSV signal model (serde)
    - trajectory.rs         Trajectory model
- Makefile                  Optional Valhalla helper targets
- LICENSE                   MIT License
- README.md                 This file

# Testing
- There are currently no automated tests in this repository. TODO: Add unit tests for ETL and DB layers, plus integration test for the full build command.

# Examples
- Build database with progress output:
  cargo run -- --verbose build

# Troubleshooting
- git: command not found
  - Install Git and ensure it’s on your PATH.
- Network/clone errors
  - Ensure you can access the upstream repositories and your network/firewall permits cloning.
- File not found errors for Excel/ZIP inputs
  - Verify the repos were cloned to --repo-path and expected files exist as listed above.
- SQLITE_BUSY or file locks
  - Close any process using the database file and retry.

# License
This project is licensed under the MIT License. See LICENSE for details.

# Notes and TODOs
- TODO: Document data dictionary for signals/vehicles with columns and units.
- TODO: Provide sample queries and expected row counts for a sanity check.
- TODO: Add tests and CI configuration.
