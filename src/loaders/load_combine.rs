use polars::prelude::*;

use crate::downloader::download_nflverse;
use crate::error::Result;

/// Load NFL combine data.
///
/// If `seasons` is None, loads all seasons.
/// Pass a vec of season years to filter to specific seasons.
pub fn load_combine(seasons: Option<Vec<i32>>) -> Result<DataFrame> {
    let mut df = download_nflverse("combine/combine")?;

    if let Some(season_list) = seasons {
        let season_series = Series::new(PlSmallStr::from("seasons"), &season_list);
        df = df
            .lazy()
            .filter(col("season").is_in(lit(season_series)))
            .collect()?;
    }

    Ok(df)
}
