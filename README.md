# evedb (Rust)

Build the eVED (Extended Vehicle Energy Dataset) SQLite database locally from the upstream source data repositories.

## Overview

This command-line tool automates cloning the source data, extracting and transforming vehicle and signal data, and loading it into a local SQLite database. It can also perform map-matching of trajectories using a Valhalla instance.

## Key Features

- **Automated Data Pipeline**: One-command build from raw source data to a processed SQLite database.
- **Fast Processing**: Utilizes Rust's performance for CSV handling, XLSX parsing, and bulk database inserts with transactions.
- **Geospatial Enrichment**: Includes H3 indexing and map-matching of vehicle trajectories.
- **Interactive Mode**: Provides a user-friendly CLI menu for configuration and command execution.
- **Flexible Configuration**: Persists settings (paths) in a local `evedb.json` file and supports environment variable overrides.

## Stack and Tooling

- **Language**: Rust (edition 2024)
- **Package Manager**: Cargo
- **Async Runtime**: `tokio`
- **CLI**: `clap` for arguments, `inquire` for interactive menus
- **Database**: `rusqlite` (bundled, no external SQLite dependency required)
- **Data Handling**: `csv`, `calamine` (XLSX), `zip`, `serde`, `serde_json`
- **Geospatial**: `h3o` (H3 indexing), `geo`
- **External Integration**: `valhalla-client` (for map-matching)
- **Utilities**: `indicatif` (progress bars), `chrono`, `rayon` (parallel processing)

## Requirements

- **Rust toolchain**: Stable Rust (compatible with edition 2024).
- **Git**: Required to clone source datasets.
- **Internet access**: To fetch the datasets:
  - [eved_dataset (Bitbucket)](https://bitbucket.org/datarepo/eved_dataset.git)
  - [VED (GitHub)](https://github.com/gsoh/VED.git)
- **Optional**: Docker or Podman (for running a local Valhalla instance).

## Installation

1.  **Clone this repository**:
    ```bash
    git clone <this-repo-url>
    cd evedb
    ```
2.  **Build the project**:
    ```bash
    cargo build --release
    ```

## Usage

The binary `evedb` provides several subcommands. If run without arguments, it defaults to **Interactive Mode**.

### Global Options

- `--repo-path <PATH>`: Directory where datasets are cloned. (Default: `./data/eved/repo`)
- `--db-path <FILE>`: Path to the output SQLite database. (Default: `./data/eved/evedb.db`)
- `--verbose`: Enable verbose output.

### Subcommands

- `interactive` (Default): Opens an interactive menu to configure paths and run operations.
- `build [--no-clone] [--no-clean]`: Full pipeline: clone, load data into DB, and (optionally) clean up.
- `match`: Map-matches loaded trajectories using a Valhalla service.
- `clone`: Only clones the source repositories to the specified path.
- `clean`: Removes the cloned repositories.

### Example Commands

```bash
# Start interactive mode
cargo run -- interactive

# Build the database from scratch
cargo run -- build

# Build using already cloned data and keep the source files
cargo run -- build --no-clone --no-clean

# Map-match trajectories (requires Valhalla running)
cargo run -- match
```

## Configuration

- **Config File**: On the first run, the tool creates `evedb.json` in the project root to store your `repo_path` and `db_path`. Subsequent runs will load these defaults.
- **Environment Variables**:
  - `VALHALLA_URL`: URL of the Valhalla instance for map-matching. (Default: `http://localhost:8002/`)

## Project Structure

- `src/main.rs`: Entry point.
- `src/cli.rs`: CLI argument and subcommand definitions.
- `src/commands/`: Implementation of subcommands (`build`, `clone`, `match`, etc.).
- `src/db/`: SQLite schema definitions and database interaction logic.
- `src/etl/`: Extraction, Transformation, and Loading logic.
- `src/models/`: Internal data models and configuration logic.
- `Makefile`: Convenience scripts for development and Valhalla orchestration.
- `docs/`: Additional documentation (e.g., [Data Dictionary](docs/data_dictionary.md)).

## Development and Scripts

The `Makefile` includes several targets to streamline development:

- `make build`: Run the build pipeline with development paths.
- `make match`: Run map-matching.
- `make docker-run` / `make podman-run`: Start a local Valhalla container pre-configured for Michigan data.
- `make get-map`: Download sample OSM data for Michigan.
- `make flamegraph` / `make samply`: Profiling tools.

### Testing

The project includes unit and integration tests.

```bash
cargo test
```

## Data Sources

The build process expects:
- **eved_dataset**: `{repo_path}/eved/data/eved.zip`
- **VED**: `{repo_path}/ved/Data/VED_Static_Data_*.xlsx`

## License

This project is licensed under the MIT License. See `LICENSE` for details.
