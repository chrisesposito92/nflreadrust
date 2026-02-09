# nflreadrust

Rust port of nflreadpy (https://github.com/nflverse/nflreadpy) - reads NFL data from nflverse GitHub releases.

## Build & Test
- `PATH="$HOME/.cargo/bin:$PATH" cargo build` - build (do NOT use `source .cargo/env`, it fails in this shell)
- `PATH="$HOME/.cargo/bin:$PATH" cargo test` - run all tests (49 unit tests + 1 data verification)
- `PATH="$HOME/.cargo/bin:$PATH" cargo test --test verify_data -- --nocapture` - verify all loaders return real data with row counts
- Tests require network access (downloads from GitHub releases)

## Architecture
- `src/lib.rs` - public API re-exports all loader functions
- `src/config.rs` - config singleton (cache mode, timeout, env vars prefixed `NFLREADRUST_`)
- `src/cache.rs` - memory + filesystem caching with MD5 keys
- `src/downloader.rs` - HTTP download + parquet/CSV parsing
- `src/utils_date.rs` - NFL season/week calculation
- `src/loaders/` - one file per data type, all return `polars::DataFrame`
- `src/loaders/seasons.rs` - shared season resolution logic

## Polars 0.46 Gotchas
- CSV: use `CsvReadOptions::default().with_parse_options(parse_opts).into_reader_with_file_handle(cursor).finish()` — NOT `CsvReader::new().with_null_values()`
- `NullValues::AllColumns` takes `Vec<PlSmallStr>` not `Vec<String>`
- `col.i32()?.into_iter().flatten().min()` for min/max — `ChunkAgg` trait not in prelude
- `Expr::is_in()` requires `is_in` feature flag
- NFL data needs feature flags: `dtype-struct`, `dtype-array`, `dtype-categorical`
- Multi-season concat: `polars::prelude::concat(lazy_frames, UnionArgs { diagonal: true, to_supertypes: true, .. })`

## Data Sources
- nflverse-data: `https://github.com/nflverse/nflverse-data/releases/download/`
- dynastyprocess: `https://github.com/dynastyprocess/data/raw/master/files/`
- ffopportunity: `https://github.com/ffverse/ffopportunity/releases/download/`
