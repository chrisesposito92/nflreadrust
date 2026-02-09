use polars::prelude::DataFrame;

use crate::downloader::download_nflverse;
use crate::error::Result;

/// Load player information. No season parameter needed.
pub fn load_players() -> Result<DataFrame> {
    download_nflverse("players/players")
}
