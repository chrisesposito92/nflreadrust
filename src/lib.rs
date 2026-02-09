pub mod cache;
pub mod config;
pub mod downloader;
pub mod error;
pub mod loaders;
pub mod utils_date;

pub use config::{CacheMode, Config};
pub use error::NflReadError;
pub use utils_date::{get_current_season, get_current_week};

// Re-export all loader functions at the crate root
pub use loaders::load_combine::load_combine;
pub use loaders::load_contracts::load_contracts;
pub use loaders::load_depth_charts::load_depth_charts;
pub use loaders::load_draft_picks::load_draft_picks;
pub use loaders::load_ffverse::{load_ff_opportunity, load_ff_playerids, load_ff_rankings};
pub use loaders::load_ftn_charting::load_ftn_charting;
pub use loaders::load_injuries::load_injuries;
pub use loaders::load_nextgen_stats::load_nextgen_stats;
pub use loaders::load_officials::load_officials;
pub use loaders::load_participation::load_participation;
pub use loaders::load_pbp::load_pbp;
pub use loaders::load_pfr_advstats::load_pfr_advstats;
pub use loaders::load_player_stats::load_player_stats;
pub use loaders::load_players::load_players;
pub use loaders::load_rosters::load_rosters;
pub use loaders::load_rosters_weekly::load_rosters_weekly;
pub use loaders::load_schedules::load_schedules;
pub use loaders::load_snap_counts::load_snap_counts;
pub use loaders::load_team_stats::load_team_stats;
pub use loaders::load_teams::load_teams;
pub use loaders::load_trades::load_trades;

pub use cache::clear_cache;
