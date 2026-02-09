use nflreadrust::*;

#[test]
fn test_get_current_season_standard() {
    let season = get_current_season(false);
    assert!(season >= 2024);
    assert!(season <= 2030);
}

#[test]
fn test_get_current_season_roster() {
    let season = get_current_season(true);
    assert!(season >= 2024);
    assert!(season <= 2030);
}

#[test]
fn test_config_default() {
    let config = Config::default();
    assert_eq!(config.cache_mode, CacheMode::Memory);
    assert_eq!(config.cache_duration, 86400);
    assert!(!config.verbose);
    assert_eq!(config.timeout, 120);
}

#[test]
fn test_cache_mode_from_str() {
    assert_eq!(CacheMode::from_str("memory"), CacheMode::Memory);
    assert_eq!(CacheMode::from_str("filesystem"), CacheMode::Filesystem);
    assert_eq!(CacheMode::from_str("off"), CacheMode::Off);
    assert_eq!(CacheMode::from_str("MEMORY"), CacheMode::Memory);
    assert_eq!(CacheMode::from_str("unknown"), CacheMode::Memory);
}

// Helper to get min/max of i32 column
fn col_i32_min(df: &polars::prelude::DataFrame, col_name: &str) -> i32 {
    let col = df.column(col_name).unwrap();
    col.i32().unwrap().into_iter().flatten().min().unwrap()
}

fn col_i32_max(df: &polars::prelude::DataFrame, col_name: &str) -> i32 {
    let col = df.column(col_name).unwrap();
    col.i32().unwrap().into_iter().flatten().max().unwrap()
}

// --- Integration tests that require network access ---

#[test]
fn test_load_teams() {
    let df = load_teams().expect("Failed to load teams");
    assert!(df.height() > 0, "Teams DataFrame should not be empty");
    assert!(
        df.get_column_names()
            .contains(&&polars::prelude::PlSmallStr::from("team_abbr")),
        "Should contain team_abbr column"
    );
}

#[test]
fn test_load_players() {
    let df = load_players().expect("Failed to load players");
    assert!(df.height() > 0, "Players DataFrame should not be empty");
}

#[test]
fn test_load_contracts() {
    let df = load_contracts().expect("Failed to load contracts");
    assert!(df.height() > 0, "Contracts DataFrame should not be empty");
}

#[test]
fn test_load_trades() {
    let df = load_trades().expect("Failed to load trades");
    assert!(df.height() > 0, "Trades DataFrame should not be empty");
}

#[test]
fn test_load_schedules_all() {
    let df = load_schedules(None).expect("Failed to load schedules");
    assert!(df.height() > 0, "Schedules DataFrame should not be empty");
    let cols = df.get_column_names();
    assert!(cols.contains(&&polars::prelude::PlSmallStr::from("season")));
    assert!(cols.contains(&&polars::prelude::PlSmallStr::from("game_id")));
}

#[test]
fn test_load_schedules_filtered() {
    let df = load_schedules(Some(vec![2023])).expect("Failed to load schedules for 2023");
    assert!(df.height() > 0);
    assert_eq!(col_i32_min(&df, "season"), 2023);
    assert_eq!(col_i32_max(&df, "season"), 2023);
}

#[test]
fn test_load_draft_picks() {
    let df = load_draft_picks(Some(vec![2023])).expect("Failed to load draft picks");
    assert!(df.height() > 0, "Draft picks should not be empty");
}

#[test]
fn test_load_combine() {
    let df = load_combine(Some(vec![2023])).expect("Failed to load combine");
    assert!(df.height() > 0, "Combine should not be empty");
}

#[test]
fn test_load_officials() {
    let df = load_officials(Some(vec![2023])).expect("Failed to load officials");
    assert!(df.height() > 0, "Officials should not be empty");
}

#[test]
fn test_load_pbp_single_season() {
    let df = load_pbp(Some(vec![2023])).expect("Failed to load PBP");
    assert!(df.height() > 0, "PBP should not be empty");
    let cols = df.get_column_names();
    assert!(cols.contains(&&polars::prelude::PlSmallStr::from("play_id")));
}

#[test]
fn test_load_player_stats_week() {
    let df =
        load_player_stats(Some(vec![2023]), "week").expect("Failed to load player stats week");
    assert!(df.height() > 0);
}

#[test]
fn test_load_player_stats_reg() {
    let df =
        load_player_stats(Some(vec![2023]), "reg").expect("Failed to load player stats reg");
    assert!(df.height() > 0);
}

#[test]
fn test_load_player_stats_invalid_level() {
    let result = load_player_stats(Some(vec![2023]), "invalid");
    assert!(result.is_err());
}

#[test]
fn test_load_team_stats_week() {
    let df = load_team_stats(Some(vec![2023]), "week").expect("Failed to load team stats week");
    assert!(df.height() > 0);
}

#[test]
fn test_load_rosters() {
    let df = load_rosters(Some(vec![2023])).expect("Failed to load rosters");
    assert!(df.height() > 0, "Rosters should not be empty");
}

#[test]
fn test_load_rosters_weekly() {
    let df = load_rosters_weekly(Some(vec![2023])).expect("Failed to load weekly rosters");
    assert!(df.height() > 0);
}

#[test]
fn test_load_snap_counts() {
    let df = load_snap_counts(Some(vec![2023])).expect("Failed to load snap counts");
    assert!(df.height() > 0);
}

#[test]
fn test_load_injuries() {
    let df = load_injuries(Some(vec![2023])).expect("Failed to load injuries");
    assert!(df.height() > 0);
}

#[test]
fn test_load_depth_charts() {
    let df = load_depth_charts(Some(vec![2023])).expect("Failed to load depth charts");
    assert!(df.height() > 0);
}

#[test]
fn test_load_ftn_charting() {
    let df = load_ftn_charting(Some(vec![2023])).expect("Failed to load FTN charting");
    assert!(df.height() > 0);
}

#[test]
fn test_load_nextgen_stats_passing() {
    let df = load_nextgen_stats(Some(vec![2023]), "passing")
        .expect("Failed to load nextgen stats passing");
    assert!(df.height() > 0);
}

#[test]
fn test_load_nextgen_stats_receiving() {
    let df = load_nextgen_stats(Some(vec![2023]), "receiving")
        .expect("Failed to load nextgen stats receiving");
    assert!(df.height() > 0);
}

#[test]
fn test_load_nextgen_stats_rushing() {
    let df = load_nextgen_stats(Some(vec![2023]), "rushing")
        .expect("Failed to load nextgen stats rushing");
    assert!(df.height() > 0);
}

#[test]
fn test_load_nextgen_stats_invalid_type() {
    let result = load_nextgen_stats(Some(vec![2023]), "invalid");
    assert!(result.is_err());
}

#[test]
fn test_load_pfr_advstats_week() {
    let df = load_pfr_advstats(Some(vec![2023]), "pass", "week")
        .expect("Failed to load PFR advstats week");
    assert!(df.height() > 0);
}

#[test]
fn test_load_pfr_advstats_season() {
    let df = load_pfr_advstats(Some(vec![2023]), "pass", "season")
        .expect("Failed to load PFR advstats season");
    assert!(df.height() > 0);
}

#[test]
fn test_load_pfr_advstats_all_types() {
    for stat_type in &["pass", "rush", "rec", "def"] {
        let df = load_pfr_advstats(Some(vec![2023]), stat_type, "week")
            .unwrap_or_else(|_| panic!("Failed to load PFR advstats {stat_type}"));
        assert!(
            df.height() > 0,
            "PFR advstats {stat_type} should not be empty"
        );
    }
}

#[test]
fn test_load_pfr_advstats_invalid_type() {
    let result = load_pfr_advstats(Some(vec![2023]), "invalid", "week");
    assert!(result.is_err());
}

#[test]
fn test_load_pfr_advstats_invalid_level() {
    let result = load_pfr_advstats(Some(vec![2023]), "pass", "invalid");
    assert!(result.is_err());
}

#[test]
fn test_load_ff_playerids() {
    let df = load_ff_playerids().expect("Failed to load FF player IDs");
    assert!(df.height() > 0);
}

#[test]
fn test_load_ff_rankings_draft() {
    let df = load_ff_rankings("draft").expect("Failed to load FF rankings draft");
    assert!(df.height() > 0);
}

#[test]
fn test_load_ff_rankings_invalid_type() {
    let result = load_ff_rankings("invalid");
    assert!(result.is_err());
}

#[test]
fn test_load_pbp_multi_season() {
    let df = load_pbp(Some(vec![2022, 2023])).expect("Failed to load PBP multi-season");
    assert!(df.height() > 0);
    assert_eq!(col_i32_min(&df, "season"), 2022);
    assert_eq!(col_i32_max(&df, "season"), 2023);
}

#[test]
fn test_clear_cache() {
    // Should not panic
    clear_cache(None);
}

#[test]
fn test_load_schedules_roof_cleaning() {
    let df = load_schedules(None).expect("Failed to load schedules");
    if df
        .get_column_names()
        .contains(&&polars::prelude::PlSmallStr::from("roof"))
    {
        let roof_col = df.column("roof").unwrap();
        let valid = &["dome", "outdoors", "closed", "open"];
        let non_null = roof_col.str().unwrap();
        for val in non_null.into_iter().flatten() {
            assert!(valid.contains(&val), "Invalid roof value: {val}");
        }
    }
}

#[test]
fn test_get_current_week_date_based() {
    let week = get_current_week(true).expect("Failed to get current week");
    assert!(
        (1..=22).contains(&week),
        "Week should be between 1 and 22, got {week}"
    );
}

#[test]
fn test_get_current_week_schedule_based() {
    let week = get_current_week(false).expect("Failed to get current week from schedule");
    assert!(
        (1..=22).contains(&week),
        "Week should be between 1 and 22, got {week}"
    );
}

#[test]
fn test_load_participation() {
    // Use 2023 which should be a completed season
    let df = load_participation(Some(vec![2023])).expect("Failed to load participation");
    assert!(df.height() > 0);
}

#[test]
fn test_load_ff_opportunity() {
    let df = load_ff_opportunity(Some(vec![2023]), "weekly", "latest")
        .expect("Failed to load FF opportunity");
    assert!(df.height() > 0);
}

#[test]
fn test_load_ff_rankings_week() {
    let df = load_ff_rankings("week").expect("Failed to load FF rankings week");
    assert!(df.height() > 0);
}

#[test]
fn test_load_ff_rankings_all() {
    let df = load_ff_rankings("all").expect("Failed to load FF rankings all");
    assert!(df.height() > 0);
}

#[test]
fn test_load_player_stats_post() {
    let df = load_player_stats(Some(vec![2023]), "post")
        .expect("Failed to load player stats post");
    assert!(df.height() > 0);
}

#[test]
fn test_load_player_stats_regpost() {
    let df = load_player_stats(Some(vec![2023]), "reg+post")
        .expect("Failed to load player stats reg+post");
    assert!(df.height() > 0);
}

#[test]
fn test_load_team_stats_reg() {
    let df = load_team_stats(Some(vec![2023]), "reg")
        .expect("Failed to load team stats reg");
    assert!(df.height() > 0);
}

#[test]
fn test_invalid_season_before_first() {
    // PBP first season is 1999
    let result = load_pbp(Some(vec![1990]));
    assert!(result.is_err());
}
