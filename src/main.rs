use std::error::Error;

use advent_of_code_2024::{day1, day2};

fn main() -> Result<(), Box<dyn Error + 'static>> {
    // day1::total_list_distance()?;
    // day1::similarity_score()?;

    day2::safe_reports()?;
    day2::safe_reports_dampened()?;
    Ok(())
}
