use polars::prelude::DataFrame;

use crate::downloader::download_nflverse;
use crate::error::Result;

/// Load historical contract data. No season parameter needed.
pub fn load_contracts() -> Result<DataFrame> {
    download_nflverse("contracts/historical_contracts")
}
