use polars::prelude::*;

use crate::downloader::download_nflverse;
use crate::error::{NflReadError, Result};
use crate::loaders::seasons::resolve_seasons;

const FIRST_SEASON: i32 = 2018;

/// Load Pro Football Reference advanced stats.
///
/// `stat_type` can be: "pass", "rush", "rec", or "def"
/// `summary_level` can be: "week" or "season"
///
/// For "week" level, downloads one file per season.
/// For "season" level, downloads a single combined file and filters by season.
pub fn load_pfr_advstats(
    seasons: Option<Vec<i32>>,
    stat_type: &str,
    summary_level: &str,
) -> Result<DataFrame> {
    validate_stat_type(stat_type)?;
    validate_summary_level(summary_level)?;

    match summary_level {
        "week" => load_weekly(seasons, stat_type),
        "season" => load_season(seasons, stat_type),
        _ => unreachable!(),
    }
}

fn load_weekly(seasons: Option<Vec<i32>>, stat_type: &str) -> Result<DataFrame> {
    let season_list = resolve_seasons(seasons, FIRST_SEASON)?;
    let mut dfs = Vec::new();

    for season in &season_list {
        let path = format!("pfr_advstats/advstats_week_{stat_type}_{season}");
        let df = download_nflverse(&path)?;
        dfs.push(df);
    }

    if dfs.len() == 1 {
        return Ok(dfs.into_iter().next().unwrap());
    }

    let lazy_frames: Vec<LazyFrame> = dfs.into_iter().map(|df| df.lazy()).collect();
    Ok(concat(lazy_frames, UnionArgs {
        parallel: true,
        rechunk: true,
        to_supertypes: true,
        diagonal: true,
        from_partitioned_ds: false,
        maintain_order: true,
    })?.collect()?)
}

fn load_season(seasons: Option<Vec<i32>>, stat_type: &str) -> Result<DataFrame> {
    let path = format!("pfr_advstats/advstats_season_{stat_type}");
    let mut df = download_nflverse(&path)?;

    if let Some(season_list) = seasons {
        for s in &season_list {
            if *s < FIRST_SEASON {
                return Err(NflReadError::InvalidSeason(format!(
                    "Season {s} is before first available season ({FIRST_SEASON})"
                )));
            }
        }
        let season_series = Series::new(PlSmallStr::from("seasons"), &season_list);
        df = df
            .lazy()
            .filter(col("season").is_in(lit(season_series)))
            .collect()?;
    }

    Ok(df)
}

fn validate_stat_type(stat_type: &str) -> Result<()> {
    match stat_type {
        "pass" | "rush" | "rec" | "def" => Ok(()),
        _ => Err(NflReadError::InvalidParameter(format!(
            "Invalid stat_type: '{stat_type}'. Must be one of: pass, rush, rec, def"
        ))),
    }
}

fn validate_summary_level(level: &str) -> Result<()> {
    match level {
        "week" | "season" => Ok(()),
        _ => Err(NflReadError::InvalidParameter(format!(
            "Invalid summary_level: '{level}'. Must be one of: week, season"
        ))),
    }
}
