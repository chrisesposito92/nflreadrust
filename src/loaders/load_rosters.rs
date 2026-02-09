use polars::prelude::*;

use crate::downloader::download_nflverse;
use crate::error::Result;
use crate::loaders::seasons::resolve_seasons_roster;

const FIRST_SEASON: i32 = 1920;

/// Load roster data for the given seasons.
///
/// Uses the roster cutoff (March 15) for determining the current season.
pub fn load_rosters(seasons: Option<Vec<i32>>) -> Result<DataFrame> {
    let season_list = resolve_seasons_roster(seasons, FIRST_SEASON)?;
    let mut dfs = Vec::new();

    for season in &season_list {
        let path = format!("rosters/roster_{season}");
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
