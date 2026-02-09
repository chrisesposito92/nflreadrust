use nflreadrust::*;

fn report(name: &str, df: &polars::prelude::DataFrame) {
    let cols: Vec<String> = df
        .get_column_names()
        .iter()
        .take(5)
        .map(|c| c.to_string())
        .collect();
    println!(
        "{:<30} rows={:<8} cols={:<4} first_cols={:?}",
        name,
        df.height(),
        df.width(),
        cols
    );
}

#[test]
fn verify_all_loaders_return_real_data() {
    println!("\n{:=<80}", "");
    println!("VERIFYING ALL LOADERS RETURN REAL DATA");
    println!("{:=<80}\n", "");

    // No-param loaders
    let df = load_teams().expect("load_teams failed");
    report("load_teams", &df);
    assert!(df.height() >= 32, "Expected at least 32 NFL teams, got {}", df.height());

    let df = load_players().expect("load_players failed");
    report("load_players", &df);
    assert!(df.height() > 1000, "Expected many players, got {}", df.height());

    let df = load_contracts().expect("load_contracts failed");
    report("load_contracts", &df);
    assert!(df.height() > 100, "Expected many contracts, got {}", df.height());

    let df = load_trades().expect("load_trades failed");
    report("load_trades", &df);
    assert!(df.height() > 10, "Expected some trades, got {}", df.height());

    // Single-file + filter loaders
    let df = load_schedules(Some(vec![2023])).expect("load_schedules failed");
    report("load_schedules(2023)", &df);
    assert!(df.height() >= 200, "Expected 200+ games in 2023, got {}", df.height());

    let df = load_draft_picks(Some(vec![2023])).expect("load_draft_picks failed");
    report("load_draft_picks(2023)", &df);
    assert!(df.height() >= 200, "Expected 200+ picks in 2023, got {}", df.height());

    let df = load_combine(Some(vec![2023])).expect("load_combine failed");
    report("load_combine(2023)", &df);
    assert!(df.height() > 50, "Expected 50+ combine entries, got {}", df.height());

    let df = load_officials(Some(vec![2023])).expect("load_officials failed");
    report("load_officials(2023)", &df);
    assert!(df.height() > 100, "Expected 100+ official entries, got {}", df.height());

    // Season-iterated loaders
    let df = load_pbp(Some(vec![2023])).expect("load_pbp failed");
    report("load_pbp(2023)", &df);
    assert!(df.height() > 40000, "Expected 40k+ plays in 2023, got {}", df.height());

    let df = load_player_stats(Some(vec![2023]), "week").expect("load_player_stats week failed");
    report("load_player_stats(week)", &df);
    assert!(df.height() > 1000, "Expected 1000+ player stat rows, got {}", df.height());

    let df = load_player_stats(Some(vec![2023]), "reg").expect("load_player_stats reg failed");
    report("load_player_stats(reg)", &df);
    assert!(df.height() > 100, "Expected 100+ player season stats, got {}", df.height());

    let df = load_player_stats(Some(vec![2023]), "post").expect("load_player_stats post failed");
    report("load_player_stats(post)", &df);
    assert!(df.height() > 10, "Expected some postseason stats, got {}", df.height());

    let df = load_player_stats(Some(vec![2023]), "reg+post").expect("load_player_stats reg+post failed");
    report("load_player_stats(reg+post)", &df);
    assert!(df.height() > 100, "Expected 100+ combined stats, got {}", df.height());

    let df = load_team_stats(Some(vec![2023]), "week").expect("load_team_stats week failed");
    report("load_team_stats(week)", &df);
    assert!(df.height() > 500, "Expected 500+ team stat rows, got {}", df.height());

    let df = load_team_stats(Some(vec![2023]), "reg").expect("load_team_stats reg failed");
    report("load_team_stats(reg)", &df);
    assert!(df.height() >= 32, "Expected 32+ team season stats, got {}", df.height());

    let df = load_rosters(Some(vec![2023])).expect("load_rosters failed");
    report("load_rosters(2023)", &df);
    assert!(df.height() > 1500, "Expected 1500+ roster entries, got {}", df.height());

    let df = load_rosters_weekly(Some(vec![2023])).expect("load_rosters_weekly failed");
    report("load_rosters_weekly(2023)", &df);
    assert!(df.height() > 10000, "Expected 10k+ weekly roster rows, got {}", df.height());

    let df = load_snap_counts(Some(vec![2023])).expect("load_snap_counts failed");
    report("load_snap_counts(2023)", &df);
    assert!(df.height() > 5000, "Expected 5k+ snap count rows, got {}", df.height());

    let df = load_injuries(Some(vec![2023])).expect("load_injuries failed");
    report("load_injuries(2023)", &df);
    assert!(df.height() > 500, "Expected 500+ injury rows, got {}", df.height());

    let df = load_depth_charts(Some(vec![2023])).expect("load_depth_charts failed");
    report("load_depth_charts(2023)", &df);
    assert!(df.height() > 5000, "Expected 5k+ depth chart rows, got {}", df.height());

    let df = load_ftn_charting(Some(vec![2023])).expect("load_ftn_charting failed");
    report("load_ftn_charting(2023)", &df);
    assert!(df.height() > 1000, "Expected 1k+ charting rows, got {}", df.height());

    let df = load_participation(Some(vec![2023])).expect("load_participation failed");
    report("load_participation(2023)", &df);
    assert!(df.height() > 30000, "Expected 30k+ participation rows, got {}", df.height());

    // Nextgen stats (single file, filtered)
    let df = load_nextgen_stats(Some(vec![2023]), "passing").expect("load_nextgen_stats passing failed");
    report("load_nextgen_stats(pass)", &df);
    assert!(df.height() > 100, "Expected 100+ NGS passing rows, got {}", df.height());

    let df = load_nextgen_stats(Some(vec![2023]), "receiving").expect("load_nextgen_stats receiving failed");
    report("load_nextgen_stats(recv)", &df);
    assert!(df.height() > 100, "Expected 100+ NGS receiving rows, got {}", df.height());

    let df = load_nextgen_stats(Some(vec![2023]), "rushing").expect("load_nextgen_stats rushing failed");
    report("load_nextgen_stats(rush)", &df);
    assert!(df.height() > 100, "Expected 100+ NGS rushing rows, got {}", df.height());

    // PFR advanced stats
    let df = load_pfr_advstats(Some(vec![2023]), "pass", "week").expect("pfr pass week failed");
    report("load_pfr_advstats(pass,wk)", &df);
    assert!(df.height() > 200, "Expected 200+ PFR pass week rows, got {}", df.height());

    let df = load_pfr_advstats(Some(vec![2023]), "rush", "week").expect("pfr rush week failed");
    report("load_pfr_advstats(rush,wk)", &df);
    assert!(df.height() > 200, "Expected 200+ PFR rush week rows, got {}", df.height());

    let df = load_pfr_advstats(Some(vec![2023]), "rec", "week").expect("pfr rec week failed");
    report("load_pfr_advstats(rec,wk)", &df);
    assert!(df.height() > 200, "Expected 200+ PFR rec week rows, got {}", df.height());

    let df = load_pfr_advstats(Some(vec![2023]), "def", "week").expect("pfr def week failed");
    report("load_pfr_advstats(def,wk)", &df);
    assert!(df.height() > 200, "Expected 200+ PFR def week rows, got {}", df.height());

    let df = load_pfr_advstats(Some(vec![2023]), "pass", "season").expect("pfr pass season failed");
    report("load_pfr_advstats(pass,szn)", &df);
    assert!(df.height() > 30, "Expected 30+ PFR pass season rows, got {}", df.height());

    // Fantasy football loaders
    let df = load_ff_playerids().expect("load_ff_playerids failed");
    report("load_ff_playerids", &df);
    assert!(df.height() > 1000, "Expected 1000+ FF player IDs, got {}", df.height());

    let df = load_ff_rankings("draft").expect("load_ff_rankings draft failed");
    report("load_ff_rankings(draft)", &df);
    assert!(df.height() > 50, "Expected 50+ draft rankings, got {}", df.height());

    let df = load_ff_rankings("week").expect("load_ff_rankings week failed");
    report("load_ff_rankings(week)", &df);
    assert!(df.height() > 50, "Expected 50+ weekly rankings, got {}", df.height());

    let df = load_ff_rankings("all").expect("load_ff_rankings all failed");
    report("load_ff_rankings(all)", &df);
    assert!(df.height() > 1000, "Expected 1000+ all rankings, got {}", df.height());

    let df = load_ff_opportunity(Some(vec![2023]), "weekly", "latest").expect("load_ff_opportunity failed");
    report("load_ff_opportunity(weekly)", &df);
    assert!(df.height() > 1000, "Expected 1000+ FF opp rows, got {}", df.height());

    // Utility functions
    let season = get_current_season(false);
    println!("\nget_current_season(false) = {season}");
    assert!(season >= 2024);

    let season_r = get_current_season(true);
    println!("get_current_season(true)  = {season_r}");
    assert!(season_r >= 2024);

    let week_d = get_current_week(true).unwrap();
    println!("get_current_week(true)    = {week_d}");
    assert!((1..=22).contains(&week_d));

    let week_s = get_current_week(false).unwrap();
    println!("get_current_week(false)   = {week_s}");
    assert!((1..=22).contains(&week_s));

    println!("\n{:=<80}", "");
    println!("ALL LOADERS VERIFIED - RETURNING REAL DATA");
    println!("{:=<80}", "");
}
