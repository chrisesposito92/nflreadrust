use polars::prelude::*;

use crate::downloader::download_nflverse;
use crate::error::Result;
use crate::loaders::seasons::resolve_seasons;

const FIRST_SEASON: i32 = 1999;

/// Load play-by-play data for the given seasons.
///
/// If `seasons` is None, loads the current season.
/// Pass a vec of season years to load specific seasons.
pub fn load_pbp(seasons: Option<Vec<i32>>) -> Result<DataFrame> {
    let season_list = resolve_seasons(seasons, FIRST_SEASON)?;
    let mut dfs = Vec::new();

    for season in &season_list {
        let path = format!("pbp/play_by_play_{season}");
        let df = download_nflverse(&path)?;
        dfs.push(df);
    }

    if dfs.len() == 1 {
        return Ok(dfs.into_iter().next().unwrap());
    }

    let refs: Vec<&DataFrame> = dfs.iter().collect();
    Ok(concat_df_diagonal(&refs)?)
}

fn concat_df_diagonal(dfs: &[&DataFrame]) -> std::result::Result<DataFrame, PolarsError> {
    let lazy_frames: Vec<LazyFrame> = dfs.iter().map(|df| (*df).clone().lazy()).collect();
    concat(lazy_frames, UnionArgs {
        parallel: true,
        rechunk: true,
        to_supertypes: true,
        diagonal: true,
        from_partitioned_ds: false,
        maintain_order: true,
    })?.collect()
}
