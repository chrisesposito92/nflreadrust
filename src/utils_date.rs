use chrono::{Datelike, Local, NaiveDate, Weekday};

/// Returns the current NFL season year.
///
/// If `roster` is true, uses March 15 as the cutoff date for the roster year.
/// If `roster` is false (default), uses the Thursday after Labor Day
/// (first Monday in September) as the cutoff.
pub fn get_current_season(roster: bool) -> i32 {
    let today = Local::now().date_naive();
    let year = today.year();

    if roster {
        let cutoff = NaiveDate::from_ymd_opt(year, 3, 15).unwrap();
        if today < cutoff { year - 1 } else { year }
    } else {
        let labor_day = first_monday_in_september(year);
        let thursday_after = labor_day + chrono::Days::new(3);
        if today < thursday_after { year - 1 } else { year }
    }
}

/// Returns the current NFL week (1-22) based on date calculation.
///
/// If `use_date` is true, calculates from the calendar.
/// If `use_date` is false, loads schedules and finds the week of the
/// next unplayed game (this requires network access).
pub fn get_current_week(use_date: bool) -> crate::error::Result<i32> {
    if use_date {
        Ok(get_current_week_from_date())
    } else {
        get_current_week_from_schedule()
    }
}

fn get_current_week_from_date() -> i32 {
    let today = Local::now().date_naive();
    let season = get_current_season(false);
    let labor_day = first_monday_in_september(season);
    let season_start = labor_day + chrono::Days::new(3); // Thursday after Labor Day

    let days_since = (today - season_start).num_days();
    if days_since < 0 {
        return 1;
    }
    let week = (days_since / 7) + 1;
    week.clamp(1, 22) as i32
}

fn get_current_week_from_schedule() -> crate::error::Result<i32> {
    let season = get_current_season(false);
    let df = crate::loaders::load_schedules::load_schedules(Some(vec![season]))?;

    // Find the first week where result is null (game not yet played)
    let mask = df.column("result")?.is_null();
    let filtered = df.filter(&mask)?;

    if filtered.height() == 0 {
        // All games played - return max week
        let week_col = df.column("week")?;
        let max_week = week_col
            .i32()?
            .into_iter()
            .flatten()
            .max()
            .unwrap_or(22);
        Ok(max_week)
    } else {
        let week_col = filtered.column("week")?;
        let min_week = week_col
            .i32()?
            .into_iter()
            .flatten()
            .min()
            .unwrap_or(1);
        Ok(min_week)
    }
}

fn first_monday_in_september(year: i32) -> NaiveDate {
    let sept1 = NaiveDate::from_ymd_opt(year, 9, 1).unwrap();
    let weekday = sept1.weekday();
    let days_to_monday = match weekday {
        Weekday::Mon => 0,
        Weekday::Tue => 6,
        Weekday::Wed => 5,
        Weekday::Thu => 4,
        Weekday::Fri => 3,
        Weekday::Sat => 2,
        Weekday::Sun => 1,
    };
    sept1 + chrono::Days::new(days_to_monday)
}
