use std::error::Error;

use advent_of_code_2024::{day1, day2, day3, day4, day5, day6, day7};

fn main() -> Result<(), Box<dyn Error + 'static>> {
    // day1::total_list_distance()?;
    // day1::similarity_score()?;

    // day2::safe_reports()?;
    // day2::safe_reports_dampened()?;

    // day3::sum_uncorrupted_mults()?;
    // day3::sum_uncorrupted_do_dont()?;

    // day4::count_xmas_count()?;
    // day4::count_masx_all()?;

    // day5::sum_middle_correctly_ordered()?;
    // day5::sum_middle_incorrect_fixed()?;

    // day6::count_distinct_block_positions()?;

    day7::total_calibration_2()?;
    Ok(())
}
