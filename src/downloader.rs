use polars::prelude::*;
use std::io::Cursor;
use std::time::Duration;

use crate::cache::{cache_get, cache_set, make_cache_key};
use crate::config::get_config;
use crate::error::{NflReadError, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataFormat {
    Parquet,
    Csv,
}

#[derive(Debug, Clone, Copy)]
pub enum Repository {
    NflverseData,
    Espnscraper,
    Dynastyprocess,
    Ffopportunity,
}

impl Repository {
    pub fn base_url(&self) -> &'static str {
        match self {
            Repository::NflverseData => {
                "https://github.com/nflverse/nflverse-data/releases/download/"
            }
            Repository::Espnscraper => {
                "https://github.com/nflverse/espnscrapeR-data/raw/master/data/"
            }
            Repository::Dynastyprocess => {
                "https://github.com/dynastyprocess/data/raw/master/files/"
            }
            Repository::Ffopportunity => {
                "https://github.com/ffverse/ffopportunity/releases/download/"
            }
        }
    }
}

pub fn build_url(repo: Repository, path: &str, format: DataFormat) -> String {
    let base = repo.base_url();
    let ext = match format {
        DataFormat::Parquet => ".parquet",
        DataFormat::Csv => ".csv",
    };
    if path.ends_with(".parquet") || path.ends_with(".csv") {
        format!("{base}{path}")
    } else {
        format!("{base}{path}{ext}")
    }
}

pub fn download_dataframe(url: &str, format: DataFormat) -> Result<DataFrame> {
    let cache_key = make_cache_key(url);

    if let Some(df) = cache_get(&cache_key) {
        return Ok(df);
    }

    let config = get_config();

    if config.verbose {
        eprintln!("Downloading: {url}");
    }

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(config.timeout))
        .user_agent(&config.user_agent)
        .build()?;

    let response = client.get(url).send()?;

    if !response.status().is_success() {
        return Err(NflReadError::Http(
            response
                .error_for_status()
                .expect_err("Expected error status"),
        ));
    }

    let bytes = response.bytes()?;
    let cursor = Cursor::new(bytes.as_ref());

    let df = match format {
        DataFormat::Parquet => ParquetReader::new(cursor).finish()?,
        DataFormat::Csv => {
            let parse_options = CsvParseOptions::default()
                .with_null_values(Some(NullValues::AllColumns(vec![
                    PlSmallStr::from("NA"),
                    PlSmallStr::from("NULL"),
                    PlSmallStr::from(""),
                ])));
            CsvReadOptions::default()
                .with_has_header(true)
                .with_parse_options(parse_options)
                .into_reader_with_file_handle(cursor)
                .finish()?
        }
    };

    cache_set(&cache_key, &df);
    Ok(df)
}

pub fn download_nflverse(path: &str) -> Result<DataFrame> {
    let url = build_url(Repository::NflverseData, path, DataFormat::Parquet);
    download_dataframe(&url, DataFormat::Parquet)
}
