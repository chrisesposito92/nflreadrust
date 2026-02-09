use polars::prelude::*;

use crate::downloader::download_nflverse;
use crate::error::Result;

/// Load schedule data.
///
/// If `seasons` is None, loads all seasons.
/// Pass a vec of season years to filter to specific seasons.
pub fn load_schedules(seasons: Option<Vec<i32>>) -> Result<DataFrame> {
    let mut df = download_nflverse("schedules/games")?;

    // Clean roof values
    let valid_roof = &["dome", "outdoors", "closed", "open"];
    if df.get_column_names().contains(&&PlSmallStr::from("roof")) {
        df = df
            .lazy()
            .with_columns([when(col("roof").is_in(lit(Series::new(
                PlSmallStr::from("roof"),
                valid_roof,
            ))))
            .then(col("roof"))
            .otherwise(lit(NULL))
            .alias("roof")])
            .collect()?;
    }

    // Filter by season if specified
    if let Some(season_list) = seasons {
        let season_series = Series::new(PlSmallStr::from("seasons"), &season_list);
        df = df
            .lazy()
            .filter(col("season").is_in(lit(season_series)))
            .collect()?;
    }

    Ok(df)
}
