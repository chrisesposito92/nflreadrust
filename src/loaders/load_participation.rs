use polars::prelude::*;

use crate::downloader::download_nflverse;
use crate::error::Result;
use crate::loaders::seasons::resolve_seasons;
use crate::utils_date::{get_current_season, get_current_week};

const FIRST_SEASON: i32 = 2016;

/// Load participation data for the given seasons.
///
/// Participation data is only available for completed seasons
/// (unless it is the final week 22 of the season).
pub fn load_participation(seasons: Option<Vec<i32>>) -> Result<DataFrame> {
    let current_week = get_current_week(false).unwrap_or(1);
    let max_season = if current_week == 22 {
        get_current_season(false)
    } else {
        get_current_season(false) - 1
    };

    let season_list = resolve_seasons(seasons, FIRST_SEASON)?;
    let mut dfs = Vec::new();

    for season in &season_list {
        if *season > max_season {
            continue;
        }
        let path = format!("pbp_participation/pbp_participation_{season}");
        let df = download_nflverse(&path)?;
        dfs.push(df);
    }

    if dfs.is_empty() {
        return Err(crate::error::NflReadError::NoData);
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
