use polars::prelude::*;

use crate::downloader::download_nflverse;
use crate::error::Result;

/// Load draft pick data.
///
/// If `seasons` is None, loads all seasons.
/// Pass a vec of season years to filter to specific seasons.
pub fn load_draft_picks(seasons: Option<Vec<i32>>) -> Result<DataFrame> {
    let mut df = download_nflverse("draft_picks/draft_picks")?;

    if let Some(season_list) = seasons {
        let season_series = Series::new(PlSmallStr::from("seasons"), &season_list);
        df = df
            .lazy()
            .filter(col("season").is_in(lit(season_series)))
            .collect()?;
    }

    Ok(df)
}
