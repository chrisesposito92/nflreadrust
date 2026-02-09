use crate::error::{NflReadError, Result};
use crate::utils_date::get_current_season;

/// Represents the `seasons` parameter that most loader functions accept.
/// - `None` => current season only
/// - `true` (All) => all available seasons from `first_season` to current
/// - Single season int
/// - List of season ints
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Seasons {
    Current,
    All,
    Single(i32),
    Multiple(Vec<i32>),
}

impl Seasons {
    /// Resolve to a concrete list of season years.
    /// `first_season` is the earliest season for which data is available.
    /// `roster` controls whether the roster cutoff date is used.
    pub fn resolve(&self, first_season: i32, roster: bool) -> Result<Vec<i32>> {
        let current = get_current_season(roster);
        match self {
            Seasons::Current => Ok(vec![current]),
            Seasons::All => {
                if first_season > current {
                    return Err(NflReadError::InvalidSeason(format!(
                        "First available season ({first_season}) is after current season ({current})"
                    )));
                }
                Ok((first_season..=current).collect())
            }
            Seasons::Single(s) => {
                if *s < first_season {
                    return Err(NflReadError::InvalidSeason(format!(
                        "Season {s} is before first available season ({first_season})"
                    )));
                }
                Ok(vec![*s])
            }
            Seasons::Multiple(seasons) => {
                for s in seasons {
                    if *s < first_season {
                        return Err(NflReadError::InvalidSeason(format!(
                            "Season {s} is before first available season ({first_season})"
                        )));
                    }
                }
                Ok(seasons.clone())
            }
        }
    }
}

/// Helper to resolve seasons and pass `roster=false` (default for most loaders).
pub fn resolve_seasons(seasons: Option<Vec<i32>>, first_season: i32) -> Result<Vec<i32>> {
    let s = match seasons {
        None => Seasons::Current,
        Some(v) => Seasons::Multiple(v),
    };
    s.resolve(first_season, false)
}

/// Helper to resolve seasons with `roster=true` (for roster-related loaders).
pub fn resolve_seasons_roster(seasons: Option<Vec<i32>>, first_season: i32) -> Result<Vec<i32>> {
    let s = match seasons {
        None => Seasons::Current,
        Some(v) => Seasons::Multiple(v),
    };
    s.resolve(first_season, true)
}
