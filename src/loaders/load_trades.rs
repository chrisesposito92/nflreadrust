use polars::prelude::DataFrame;

use crate::downloader::download_nflverse;
use crate::error::Result;

/// Load trade data. No season parameter needed.
pub fn load_trades() -> Result<DataFrame> {
    download_nflverse("trades/trades")
}
