use polars::prelude::*;

use crate::downloader::download_nflverse;
use crate::error::Result;
use crate::loaders::seasons::resolve_seasons;

const FIRST_SEASON: i32 = 2002;

/// Load weekly roster data for the given seasons.
pub fn load_rosters_weekly(seasons: Option<Vec<i32>>) -> Result<DataFrame> {
    let season_list = resolve_seasons(seasons, FIRST_SEASON)?;
    let mut dfs = Vec::new();

    for season in &season_list {
        let path = format!("weekly_rosters/roster_weekly_{season}");
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
