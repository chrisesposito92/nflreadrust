use polars::prelude::*;

use crate::downloader::download_nflverse;
use crate::error::{NflReadError, Result};

const FIRST_SEASON: i32 = 2016;

/// Load Next Gen Stats data.
///
/// `stat_type` can be: "passing", "receiving", or "rushing"
/// `seasons` filters to specific seasons after download (single file per stat type).
pub fn load_nextgen_stats(
    seasons: Option<Vec<i32>>,
    stat_type: &str,
) -> Result<DataFrame> {
    validate_stat_type(stat_type)?;

    let path = format!("nextgen_stats/ngs_{stat_type}");
    let mut df = download_nflverse(&path)?;

    // Filter by season if specified
    if let Some(season_list) = seasons {
        // Validate seasons
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
        "passing" | "receiving" | "rushing" => Ok(()),
        _ => Err(NflReadError::InvalidParameter(format!(
            "Invalid stat_type: '{stat_type}'. Must be one of: passing, receiving, rushing"
        ))),
    }
}
