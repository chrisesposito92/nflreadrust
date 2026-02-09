# nflreadrust

A Rust port of [nflreadpy](https://github.com/nflverse/nflreadpy) for reading NFL data from the [nflverse](https://github.com/nflverse) project. All data is returned as [Polars](https://pola.rs/) DataFrames.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
nflreadrust = { git = "https://github.com/nflverse/nflreadrust" }
```

## Quick Start

```rust
use nflreadrust::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the current season's play-by-play data
    let pbp = load_pbp(None)?;
    println!("Plays: {} rows x {} cols", pbp.height(), pbp.width());

    // Load specific seasons
    let schedules = load_schedules(Some(vec![2022, 2023]))?;
    println!("{}", schedules.head(Some(5)));

    // Load player stats with summary level
    let stats = load_player_stats(Some(vec![2023]), "week")?;
    println!("Player stats: {} rows", stats.height());

    Ok(())
}
```

## API Reference

### Play-by-Play & Game Data

| Function | Description |
|---|---|
| `load_pbp(seasons)` | Play-by-play data (from 1999) |
| `load_schedules(seasons)` | Game schedules and results |
| `load_participation(seasons)` | Play participation data (from 2016) |

### Player & Team Stats

| Function | Description |
|---|---|
| `load_player_stats(seasons, summary_level)` | Player stats - `summary_level`: `"week"`, `"reg"`, `"post"`, `"reg+post"` |
| `load_team_stats(seasons, summary_level)` | Team stats - same summary levels as above |
| `load_nextgen_stats(seasons, stat_type)` | Next Gen Stats - `stat_type`: `"passing"`, `"receiving"`, `"rushing"` |
| `load_pfr_advstats(seasons, stat_type, summary_level)` | PFR advanced stats - `stat_type`: `"pass"`, `"rush"`, `"rec"`, `"def"` / `summary_level`: `"week"`, `"season"` |
| `load_snap_counts(seasons)` | Snap counts (from 2012) |

### Rosters & Personnel

| Function | Description |
|---|---|
| `load_rosters(seasons)` | Season rosters (from 1920) |
| `load_rosters_weekly(seasons)` | Weekly rosters (from 2002) |
| `load_players()` | Player biographical information |
| `load_depth_charts(seasons)` | Depth charts (from 2001) |
| `load_injuries(seasons)` | Injury reports (from 2009) |
| `load_officials(seasons)` | Game officials |

### Draft, Contracts & Transactions

| Function | Description |
|---|---|
| `load_draft_picks(seasons)` | Draft picks |
| `load_combine(seasons)` | NFL combine results |
| `load_contracts()` | Historical contract data |
| `load_trades()` | Trade data |

### Team Data

| Function | Description |
|---|---|
| `load_teams()` | Team metadata, colors, and logos |

### Charting

| Function | Description |
|---|---|
| `load_ftn_charting(seasons)` | FTN charting data (from 2022) |

### Fantasy Football

| Function | Description |
|---|---|
| `load_ff_playerids()` | Fantasy player ID mappings |
| `load_ff_rankings(ranking_type)` | Fantasy rankings - `ranking_type`: `"draft"`, `"week"`, `"all"` |
| `load_ff_opportunity(seasons, stat_type, model_version)` | Fantasy opportunity data - `stat_type`: `"weekly"`, `"pbp_pass"`, `"pbp_rush"` |

### Utilities

| Function | Description |
|---|---|
| `get_current_season(roster)` | Current NFL season year. If `roster=true`, uses March 15 cutoff |
| `get_current_week(use_date)` | Current NFL week (1-22). If `use_date=false`, determines from schedule |
| `clear_cache(pattern)` | Clear cached data. `None` clears all |

### Seasons Parameter

Most loader functions accept `seasons: Option<Vec<i32>>`:

- `None` - loads the current season
- `Some(vec![2023])` - loads a single season
- `Some(vec![2022, 2023])` - loads multiple seasons

Functions that download a single combined file (`load_schedules`, `load_draft_picks`, `load_combine`, `load_officials`) filter after download. Pass `None` to get all available seasons.

## Configuration

Configuration via environment variables:

| Variable | Default | Description |
|---|---|---|
| `NFLREADRUST_CACHE` | `memory` | Cache mode: `memory`, `filesystem`, or `off` |
| `NFLREADRUST_CACHE_DIR` | Platform cache dir | Filesystem cache directory |
| `NFLREADRUST_CACHE_DURATION` | `86400` | Cache TTL in seconds (24 hours) |
| `NFLREADRUST_VERBOSE` | `false` | Print download URLs |
| `NFLREADRUST_TIMEOUT` | `120` | HTTP timeout in seconds |

Or programmatically:

```rust
use nflreadrust::config::{Config, update_config, CacheMode};

let mut config = Config::default();
config.cache_mode = CacheMode::Filesystem;
config.verbose = true;
update_config(config);
```

## Data Sources

All data is sourced from nflverse GitHub repositories:

- [nflverse-data](https://github.com/nflverse/nflverse-data) - primary data source for most functions
- [dynastyprocess](https://github.com/dynastyprocess/data) - fantasy football player IDs and rankings
- [ffopportunity](https://github.com/ffverse/ffopportunity) - fantasy football opportunity models

## License

MIT
