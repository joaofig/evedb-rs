# evedb (Rust)

Build the eVED (Extended Vehicle Energy Dataset) SQLite database locally from the upstream source data repositories.

# Overview

This command-line tool automates cloning the source data, extracting and transforming vehicle and signal data, and loading it into a local SQLite database. It can also perform map-matching of trajectories using a Valhalla instance.

## Key features
- One-command build of the eVED SQLite database.
- Fast CSV handling and bulk inserts with transactions.
- Progress feedback using `indicatif` while loading signal files.
- H3 indexing and geospatial processing with `geo`.
- Map-matching of trajectories via `valhalla-client`.
- Interactive mode for easier configuration and execution.
- Simple, discoverable CLI built with `clap`.

## Stack and tooling
- **Language**: Rust (edition 2024)
- **Package manager/build**: Cargo
- **Async runtime**: `tokio`
- **CLI**: `clap` (with `inquire` for interactive mode)
- **Database**: `rusqlite` (bundled, no external SQLite required)
- **Data handling**: `csv`, `calamine` (XLSX), `zip`, `serde`
- **Geospatial**: `h3o`, `geo`
- **External Integration**: `valhalla-client` (for map-matching)
- **HTTP client**: `reqwest` (via `valhalla-client`)
- **Utilities**: `indicatif` (progress bars), `chrono`, `rayon` (parallel processing)

## Requirements
- **Rust toolchain**: Recommended via `rustup`. Minimum stable compatible with edition 2024.
- **Git**: Required for the `clone` step.
- **Internet access**: To fetch the source datasets:
  - [eved_dataset (Bitbucket)](https://bitbucket.org/datarepo/eved_dataset.git)
  - [VED (GitHub)](https://github.com/gsoh/VED.git)
- **Optional**: Docker or Podman (only for running a local Valhalla instance via the Makefile).
- **Optional**: `cargo-flamegraph` and `samply` for profiling.

## Installation
1.  **Install Rust and Cargo**:
    - `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
    - Or follow [rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
2.  **Clone this repository**:
    ```bash
    git clone <this-repo-url>
    cd evedb
    ```

## Build
-   **Debug build**: `cargo build`
-   **Release build**: `cargo build --release`

## Run
The binary name is `evedb`. The CLI offers several subcommands: `build`, `match`, `clone`, `clean`, and `interactive`.

-   **Interactive mode** (easiest for configuration):
    `cargo run -- interactive`
-   **Typical build** (clone data, build DB, then clean repos):
    `cargo run -- build`
-   **Map-match trajectories** (requires a running Valhalla instance):
    `cargo run -- match`
-   **Build without cloning** (use existing repos at `repo_path`):
    `cargo run -- build --no-clone`
-   **Clone only**:
    `cargo run -- clone`
-   **Clean only** (remove repositories folder):
    `cargo run -- clean`

## CLI reference
Global flags and defaults:
-   `--repo-path <PATH>`: Path to the local folder where datasets will be cloned. (Default: `./data/eved/repo`)
-   `--db-path <FILE>`: Path to the output SQLite database file. (Default: `./data/eved/evedb.db`)
-   `--verbose`: Verbose logging.

### Subcommands:
-   `build [--no-clone] [--no-clean]`: Orchestrates the database creation process.
-   `match`: Map-matches the loaded trajectories using Valhalla.
-   `clone`: Clones the upstream repositories into `--repo-path`.
-   `clean`: Removes the repositories folder at `--repo-path`.
-   `interactive`: Enters an interactive menu to set paths and run commands.

## Data sources and expectations
The build process expects the following data within the cloned repositories:
-   **From eved_dataset** (cloned to `{repo_path}/eved`):
    -   `data/eVED.zip` containing CSV files for signals.
-   **From VED** (cloned to `{repo_path}/ved`):
    -   `Data/VED_Static_Data_ICE&HEV.xlsx`
    -   `Data/VED_Static_Data_PHEV&EV.xlsx`

The builder will:
1.  Create tables (`vehicles`, `signals`, `trajectories`) in the SQLite database.
2.  Load vehicle data from XLSX files.
3.  Iterate through CSV entries in `eVED.zip` and load signals.
4.  Build indexes for faster queries.
5.  Generate trajectories from loaded data.

## Environment variables
-   None are required by the application code directly.
-   `Git` must be available on `PATH` for the `clone` step.
-   Optional: `Docker`/`Podman` available on `PATH` for Valhalla helpers.

## Scripts and useful commands
The `Makefile` provides several convenience targets:
-   `make build`: Build with hardcoded paths (debug).
-   `make build-r`: Build with hardcoded paths (release).
-   `make match`: Run map-matching (debug).
-   `make match-r`: Run map-matching (release).
-   `make flamegraph`: Profile the build using `cargo-flamegraph`.
-   `make samply`: Profile the build using `samply`.
-   `make get-map`: Download a sample OSM PBF file for Michigan.
-   `make docker-run` / `make podman-run`: Start a Valhalla container.
-   `make prune-docker` / `make prune-podman`: Cleanup container system.

## Database
-   The generated SQLite database is stored at `--db-path` (default: `./data/eved/evedb.db`).
-   `rusqlite` is compiled with the `bundled` feature, so no external SQLite installation is needed.
-   See the [Data Dictionary](docs/data_dictionary.md) for details on tables, columns, and units.

# Project structure
-   `Cargo.toml`: Project manifest (name: `evedb`).
-   `src/`
    -   `main.rs`: Entry point (Tokio async main).
    -   `cli.rs`: CLI definitions (`clap`).
    -   `commands/`: Subcommand implementations (`build.rs`, `clone.rs`, `clean.rs`, `interactive.rs`).
    -   `commands/builders/`: Logic for building specific entities (e.g., `node.rs`).
    -   `db/`: SQLite schema and load routines.
    -   `etl/`: Extraction and transformation logic.
    -   `models/`: Data models for vehicles, signals, and trajectories.
-   `lib/`: Supporting libraries or modules.
-   `Makefile`: Convenience targets for development and Valhalla orchestration.
-   `LICENSE`: MIT License.

# Testing
Unit tests cover ETL and DB layers, including data parsing, transformations, and database operations.
Integration tests verify the full `build` and `match` command pipelines using temporary workspaces and a mock Valhalla server.

To run the tests:
```bash
cargo test
```

# Troubleshooting
-   **git: command not found**: Install Git and ensure it’s on your `PATH`.
-   **Network/clone errors**: Ensure you can access the upstream repositories (Bitbucket/GitHub).
-   **File not found errors**: Verify the repos were cloned correctly to `--repo-path`.
-   **SQLITE_BUSY**: Close any other process using the database file and retry.
-   **Valhalla connection error**: Ensure the Valhalla container is running (e.g., via `make docker-run`) and accessible on port 8002.

# License
This project is licensed under the MIT License. See `LICENSE` for details.

# Notes and TODOs
-   **TODO**: Provide sample queries and expected row counts for sanity checks.
