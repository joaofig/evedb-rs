# evedb - AI Assistant Guide

## Project Overview
evedb is a Rust CLI tool that builds the eVED (Extended Vehicle Energy Dataset) SQLite database from upstream source data repositories. It automates data cloning, extraction, transformation, and loading of vehicle and signal data with H3 geospatial indexing support.

## Tech Stack
- **Language**: Rust (edition 2024)
- **Runtime**: Tokio (async)
- **CLI**: clap with derive macros
- **Database**: rusqlite with bundled SQLite
- **Data Processing**: csv, calamine (XLSX), zip, serde
- **Geospatial**: h3o (H3 hexagonal hierarchical indexing), geo
- **Time**: chrono, chrono-tz
- **HTTP**: reqwest
- **Progress UI**: indicatif

## Project Structure
```
src/
├── main.rs              # Entry point with Tokio runtime
├── cli.rs               # Command-line argument definitions
├── tools.rs             # Utility functions
├── commands/
│   ├── build.rs         # Main build orchestration
│   ├── clone.rs         # Repository cloning logic
│   └── clean.rs         # Cleanup operations
├── db/
│   ├── api.rs           # Database connection wrapper
│   └── evedb.rs         # Schema, tables, indexes, queries
├── etl/extract/
│   ├── vehicles.rs      # XLSX vehicle data extraction
│   └── signals.rs       # CSV signal data extraction from ZIP
└── models/
    ├── vehicle.rs       # Vehicle data model
    ├── signal.rs        # Signal data model (CSV deserializable)
    └── trajectory.rs    # Trajectory data model
```

## Key Concepts

### Data Flow
1. **Clone**: Downloads source repositories (eved_dataset, VED)
2. **Extract Vehicles**: Reads XLSX files for vehicle metadata
3. **Extract Signals**: Processes CSV files from eVED.zip archive
4. **Build Trajectories**: Generates trajectory data with H3 indexing
5. **Index**: Creates database indexes for performance
6. **Clean**: Optionally removes cloned repositories

### Database Schema
The SQLite database contains three main tables:
- `vehicles`: Static vehicle metadata from XLSX files
- `signals`: Time-series signal data from CSV files
- `trajectories`: Derived trajectory data with H3 geospatial indexes

### H3 Integration
The project uses H3 hexagonal hierarchical spatial indexing for geospatial queries on trajectory data. H3 indexes are created during the build process to enable efficient location-based queries.

## Development Conventions

### Code Style
- Use idiomatic Rust patterns
- Prefer `anyhow::Result` for error handling in application code
- Use `?` operator for error propagation
- Async functions should use Tokio runtime

### Error Handling
- Return `anyhow::Result<()>` from command functions
- Provide context with `.context()` for better error messages
- Don't panic except in truly unrecoverable situations

### Database Operations
- Use transactions for bulk inserts
- Prepare statements for repeated queries
- Close connections explicitly when done
- Bundle progress feedback with indicatif for long operations

### Data Cleaning
- Handle NaN values in numeric data
- Clean semicolons and formatting issues in CSVs
- Validate data before insertion

## Commands

### Build
```bash
cargo run -- build [--no-clone] [--no-clean]
```
- Orchestrates the full pipeline
- `--no-clone`: Skip cloning (use existing repos)
- `--no-clean`: Keep repos after build

### Clone
```bash
cargo run -- clone
```
- Only clones the upstream repositories

### Clean
```bash
cargo run -- clean
```
- Removes the repositories folder

### Global Flags
- `--repo-path <PATH>`: Repository location (default: ./data/eved/repo)
- `--db-path <FILE>`: Database output path (default: ./data/eved/evedb.db)
- `--verbose`: Enable verbose logging

## Data Sources
- **eved_dataset** (Bitbucket): https://bitbucket.org/datarepo/eved_dataset.git
    - Contains `data/eVED.zip` with signal CSV files
- **VED** (GitHub): https://github.com/gsoh/VED.git
    - Contains XLSX files: `Data/VED_Static_Data_ICE&HEV.xlsx`, `Data/VED_Static_Data_PHEV&EV.xlsx`

## When Working on This Project

### Before Making Changes
1. Read relevant source files to understand current implementation
2. Check models for data structures
3. Review database schema in `db/evedb.rs`
4. Understand the command flow in `commands/build.rs`

### Testing Changes
- Build: `cargo build` (debug) or `cargo build --release`
- Run: `cargo run -- <command>`
- Lint: `cargo clippy`
- Format: `cargo fmt`

### Common Tasks

#### Adding a New Field to Database
1. Update the model in `models/`
2. Modify schema in `db/evedb.rs`
3. Update insert/query logic
4. Update extraction logic in `etl/extract/`

#### Adding a New Command
1. Define in `cli.rs` as new enum variant
2. Create handler in `commands/`
3. Add module to `commands/mod.rs`
4. Wire up in `main.rs`

#### Modifying Data Extraction
1. Locate extractor in `etl/extract/`
2. Update model if data structure changes
3. Update database insert logic if needed

## Important Notes
- Database uses bundled SQLite (no external installation needed)
- Git must be available on PATH for cloning
- All async operations use Tokio
- Progress bars use indicatif for user feedback
- H3 indexes are built at resolution levels appropriate for the data scale

## TODOs & Known Issues
- No automated tests yet (unit tests and integration tests needed)
- Data dictionary documentation needed
- Sample queries and row count validation needed
- CI/CD configuration needed

## Dependencies to Be Aware Of
- `h3o`: H3 geospatial indexing library
- `rusqlite`: SQLite wrapper with bundled feature
- `calamine`: Excel file reader
- `csv`: CSV parsing with serde support
- `zip`: ZIP archive handling
- `geo`: Geospatial primitives and algorithms
- `indicatif`: Terminal progress bars
- `chrono`/`chrono-tz`: Time and timezone handling

## Environment
- Minimum Rust: Stable with edition 2024 support
- Required tools: Git
- Optional tools: Docker/Podman (for Valhalla Makefile helpers only)
