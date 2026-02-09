use polars::prelude::*;

use crate::downloader::download_nflverse;
use crate::error::{NflReadError, Result};
use crate::loaders::seasons::resolve_seasons;

const FIRST_SEASON: i32 = 1999;

/// Load team stats for the given seasons.
///
/// `summary_level` can be: "week", "reg", "post", or "reg+post"
pub fn load_team_stats(
    seasons: Option<Vec<i32>>,
    summary_level: &str,
) -> Result<DataFrame> {
    validate_summary_level(summary_level)?;
    let level_str = summary_level.replace('+', "");
    let season_list = resolve_seasons(seasons, FIRST_SEASON)?;
    let mut dfs = Vec::new();

    for season in &season_list {
        let path = format!("stats_team/stats_team_{level_str}_{season}");
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

fn validate_summary_level(level: &str) -> Result<()> {
    match level {
        "week" | "reg" | "post" | "reg+post" => Ok(()),
        _ => Err(NflReadError::InvalidParameter(format!(
            "Invalid summary_level: '{level}'. Must be one of: week, reg, post, reg+post"
        ))),
    }
}
