use polars::prelude::DataFrame;

use crate::downloader::download_nflverse;
use crate::error::Result;

/// Load team metadata (colors, logos, etc.). No season parameter needed.
pub fn load_teams() -> Result<DataFrame> {
    download_nflverse("teams/teams_colors_logos")
}
