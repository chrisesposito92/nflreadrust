use thiserror::Error;

#[derive(Error, Debug)]
pub enum NflReadError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Polars error: {0}")]
    Polars(#[from] polars::prelude::PolarsError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid season: {0}")]
    InvalidSeason(String),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("No data available for the requested parameters")]
    NoData,
}

pub type Result<T> = std::result::Result<T, NflReadError>;
