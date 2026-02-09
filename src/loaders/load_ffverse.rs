use polars::prelude::*;

use crate::downloader::{DataFormat, Repository, build_url, download_dataframe};
use crate::error::{NflReadError, Result};
use crate::loaders::seasons::resolve_seasons;

/// Load fantasy football player IDs from dynastyprocess.
pub fn load_ff_playerids() -> Result<DataFrame> {
    let url = build_url(Repository::Dynastyprocess, "db_playerids.csv", DataFormat::Csv);
    download_dataframe(&url, DataFormat::Csv)
}

/// Load fantasy football rankings from dynastyprocess.
///
/// `ranking_type` can be: "draft", "week", or "all"
pub fn load_ff_rankings(ranking_type: &str) -> Result<DataFrame> {
    match ranking_type {
        "draft" => {
            let url = build_url(
                Repository::Dynastyprocess,
                "db_fpecr_latest.csv",
                DataFormat::Csv,
            );
            download_dataframe(&url, DataFormat::Csv)
        }
        "week" => {
            let url = build_url(
                Repository::Dynastyprocess,
                "fp_latest_weekly.csv",
                DataFormat::Csv,
            );
            download_dataframe(&url, DataFormat::Csv)
        }
        "all" => {
            let url = build_url(
                Repository::Dynastyprocess,
                "db_fpecr.parquet",
                DataFormat::Parquet,
            );
            download_dataframe(&url, DataFormat::Parquet)
        }
        _ => Err(NflReadError::InvalidParameter(format!(
            "Invalid ranking_type: '{ranking_type}'. Must be one of: draft, week, all"
        ))),
    }
}

/// Load fantasy football opportunity data from ffopportunity.
///
/// `stat_type` can be: "weekly", "pbp_pass", or "pbp_rush"
/// `model_version` can be: "latest" or "v1.0.0"
pub fn load_ff_opportunity(
    seasons: Option<Vec<i32>>,
    stat_type: &str,
    model_version: &str,
) -> Result<DataFrame> {
    validate_stat_type(stat_type)?;
    validate_model_version(model_version)?;

    let first_season = 2006;
    let season_list = resolve_seasons(seasons, first_season)?;
    let mut dfs = Vec::new();

    for season in &season_list {
        let path = format!("{model_version}-data/ep_{stat_type}_{season}");
        let url = build_url(Repository::Ffopportunity, &path, DataFormat::Parquet);
        let df = download_dataframe(&url, DataFormat::Parquet)?;
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

fn validate_stat_type(stat_type: &str) -> Result<()> {
    match stat_type {
        "weekly" | "pbp_pass" | "pbp_rush" => Ok(()),
        _ => Err(NflReadError::InvalidParameter(format!(
            "Invalid stat_type: '{stat_type}'. Must be one of: weekly, pbp_pass, pbp_rush"
        ))),
    }
}

fn validate_model_version(version: &str) -> Result<()> {
    match version {
        "latest" | "v1.0.0" => Ok(()),
        _ => Err(NflReadError::InvalidParameter(format!(
            "Invalid model_version: '{version}'. Must be one of: latest, v1.0.0"
        ))),
    }
}
