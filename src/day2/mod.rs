use std::{
    cmp::{max, min},
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn safe_reports() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/day2/input.txt")?;
    let mut reader = BufReader::new(file);

    let mut reports: Vec<Vec<u32>> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let line_nums: Vec<u32> = line
            .split(" ")
            .into_iter()
            .map(|elem| elem.parse::<u32>())
            .collect::<Result<_, _>>()?;
        reports.push(line_nums);
    }
    let safe_count = reports
        .iter()
        .filter(|report| {
            let mut prev = report[0];
            let mut consec_inc = true;
            let mut consec_dec = true;
            for i in 1..report.len() {
                if prev > report[i] {
                    consec_inc = false;
                    break;
                }
                prev = report[i];
            }
            prev = report[0];
            for i in 1..report.len() {
                if prev < report[i] {
                    consec_dec = false;
                    break;
                }
                prev = report[i];
            }
            if !consec_inc && !consec_dec {
                return false;
            }
            for w in report.windows(2) {
                if let [first, next] = w {
                    let diff = max(first, next) - min(first, next);
                    if diff < 1 || diff > 3 {
                        return false;
                    }
                };
            }

            true
        })
        .count();

    println!("Safe count is {}", safe_count);
    Ok(())
}

pub fn safe_reports_dampened() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/day2/input.txt")?;
    let mut reader = BufReader::new(file);

    let mut reports: Vec<Vec<i32>> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let line_nums: Vec<i32> = line
            .split(" ")
            .into_iter()
            .map(|elem| elem.parse::<i32>())
            .collect::<Result<_, _>>()?;
        reports.push(line_nums);
    }
    let safe_count = reports
        .iter()
        .filter(|nums| {
            for skip_index in 0..nums.len() {
                let mut valid = true;
                let mut curr = nums[0] - 2;
                for i in 0..nums.len() {
                    if i == skip_index {
                        if i == 0 {
                            curr = nums[1] - 2;
                        }
                        continue;
                    }
                    if nums[i] - curr < 1 || nums[i] - curr > 3 {
                        valid = false;
                        break;
                    }
                    curr = nums[i];
                }
                if valid {
                    return true;
                }
                valid = true;
                curr = nums[0] + 2;
                for i in 0..nums.len() {
                    if i == skip_index {
                        if i == 0 {
                            curr = nums[1] + 2;
                        }
                        continue;
                    }
                    if curr - nums[i] < 1 || curr - nums[i] > 3 {
                        valid = false;
                        break;
                    }
                    curr = nums[i];
                }
                if valid {
                    return true;
                }
            }
            false
        })
        .count();

    println!("Safe count (dampened) is {}", safe_count);
    Ok(())
}
