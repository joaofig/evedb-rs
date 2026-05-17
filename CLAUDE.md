# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

# evedb

evedb is a Rust CLI tool that builds the eVED (Extended Vehicle Energy Dataset) SQLite database from upstream source data repositories. It automates data cloning, extraction, transformation, and loading of vehicle and signal data with H3 geospatial indexing support.

## Tech Stack

- **Language**: Rust (edition 2024)
- **Runtime**: Tokio (async)
- **CLI**: clap with derive macros
- **Database**: rusqlite with bundled SQLite
- **Data Processing**: csv, calamine (XLSX), zip, serde
- **Geospatial**: h3o (H3 hexagonal hierarchical indexing), geo
- **HTTP**: reqwest, valhalla_client
- **Progress UI**: indicatif

## Development Commands

```bash
cargo build              # debug build
cargo build --release    # release build
cargo run -- <command>   # run a command
cargo test               # all tests
cargo test <test_name>   # single test by name
cargo clippy             # lint
cargo fmt                # format
```

### Makefile targets

| Target | Description |
|---|---|
| `make build` / `make build-r` | Debug / release build with hardcoded paths |
| `make match` / `make match-r` | Map-matching (debug / release) |
| `make get-map` | Download Michigan OSM PBF for Valhalla |
| `make docker-run` / `make podman-run` | Start Valhalla container |
| `make prune-docker` / `make prune-podman` | Remove container system data |
| `make flamegraph` / `make samply` | Profiling |
| `make update` | `cargo update --verbose` |

## Commands

Running with no subcommand defaults to interactive mode.

| Command | Description |
|---|---|
| `interactive` | Interactive menu (default when no subcommand given) |
| `build [--no-clone] [--no-clean]` | Full ETL pipeline |
| `match` | Map-match trajectories via Valhalla |
| `clone` | Clone upstream repositories only |
| `clean` | Remove the repositories folder |

**Global flags**: `--repo-path <PATH>`, `--db-path <FILE>`, `--verbose`

## Architecture

### Config persistence

On startup, `main.rs` loads `./evedb.json` (if present) via `models/config::Config::load()` and applies it to the `Cli` struct, overriding CLI defaults for `--repo-path` and `--db-path`. On exit, the current paths are saved back to `./evedb.json`. Interactive mode can update these paths in-session.

### Data flow

1. **Clone** — git-clones `eved_dataset` (Bitbucket) and `VED` (GitHub) into `repo-path`
2. **Extract vehicles** — reads XLSX files from the VED repo via `etl/extract/vehicles.rs`
3. **Extract signals** — reads CSV files from `eVED.zip` inside the eved_dataset repo via `etl/extract/signals.rs`
4. **Build trajectories** — derives trajectory rows with H3 geospatial indexes (`commands/builders/trajectory.rs`)
5. **Index** — creates SQLite indexes for query performance
6. **Map match** (optional) — `commands/builders/node.rs` calls a Valhalla instance; `etl/converters.rs` converts `TrajectoryPoint`/`WayPoint` into Valhalla `ShapePoint` via `From` impls
7. **Clean** — optionally removes cloned repos

### Database schema (four tables)

- `vehicles` — static vehicle metadata from XLSX
- `signals` — time-series signal data from CSV
- `trajectories` — derived data with H3 geospatial indexes
- `nodes` — map-matched road network nodes (Valhalla)

Schema, DDL, and queries are in `db/evedb.rs`; `db/api.rs` is the connection wrapper.

### Adding a new command

1. Add enum variant to `cli.rs`
2. Create handler in `commands/`
3. Register module in `commands/mod.rs`
4. Wire up in `main.rs`

### Adding a new database field

1. Update model in `models/`
2. Modify schema in `db/evedb.rs`
3. Update insert/query logic in the relevant builder
4. Update extractor in `etl/extract/`

## Data Sources

- **eved_dataset** (Bitbucket): `https://bitbucket.org/datarepo/eved_dataset.git` — contains `data/eVED.zip`
- **VED** (GitHub): `https://github.com/gsoh/VED.git` — contains `Data/VED_Static_Data_ICE&HEV.xlsx` and `Data/VED_Static_Data_PHEV&EV.xlsx`

## Environment Variables

- `VALHALLA_URL`: Valhalla instance URL (default: `http://localhost:8002/`)

## Testing

- Integration tests live in `tests/integration_tests.rs` and use a mock Valhalla server
- Unit tests are co-located with source in `etl/` and `db/` modules
- Git must be on `PATH` for clone-related tests to pass
