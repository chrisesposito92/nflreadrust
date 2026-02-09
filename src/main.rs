use nflreadrust::*;

fn main() {
    println!("nflreadrust - NFL data loader");
    println!("Current NFL season: {}", get_current_season(false));

    // Example: load teams data
    match load_teams() {
        Ok(df) => {
            println!("\nTeams data:");
            println!("{}", df.head(Some(5)));
        }
        Err(e) => eprintln!("Error loading teams: {e}"),
    }
}
