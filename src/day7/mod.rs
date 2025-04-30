use std::{fs::File, io::{BufRead, BufReader}};

fn possible_config(target: usize, nums: &[usize], curr: usize) -> bool {
    if nums.len() == 0 {
        return target == curr;
    }
    if curr > target {
        return false;
    }
    possible_config(target, &nums[1..], curr + nums[0]) || possible_config(target, &nums[1..], curr * nums[0])
}

fn possible_config2(target: usize, nums: &[usize], curr: usize) -> bool {
    if nums.len() == 0 {
        return target == curr;
    }
    if curr > target {
        return false;
    }
    possible_config2(target, &nums[1..], curr + nums[0]) 
    || possible_config2(target, &nums[1..], curr * nums[0])
    || possible_config2(target, &nums[1..], (curr.to_string() + &nums[0].to_string()).parse().unwrap() )
}

pub fn total_calibration() -> Result<(), anyhow::Error> {

    let input = File::open("data/day7/input.txt")?;
    let mut  reader = BufReader::new(input);
    
    let mut buffer = String::new();
    let mut sum_possible = 0;
    while let Ok(length) = reader.read_line(&mut buffer) {
        if length == 0 {
            break;
        }

        // parse line
        let tokens: Vec<&str> = buffer.split(" ").collect();
        let target: usize = tokens[0][..tokens[0].len()-1].parse()?;

        let nums: Vec<usize> = tokens[1..].into_iter().map(|str| {
            let result = str.to_owned().trim().parse::<usize>().unwrap();
            result
        }).collect();
        if possible_config(target, &nums[1..], nums[0]) {
            sum_possible += target;
        }

        buffer.clear();

    }
    println!("total calibration result: {}", sum_possible);
    
    Ok(())
}

pub fn total_calibration_2() -> Result<(), anyhow::Error> {
    let input = File::open("data/day7/input.txt")?;
    let mut  reader = BufReader::new(input);
    
    let mut buffer = String::new();
    let mut sum_possible = 0;
    while let Ok(length) = reader.read_line(&mut buffer) {
        if length == 0 {
            break;
        }

        // parse line
        let tokens: Vec<&str> = buffer.split(" ").collect();
        let target: usize = tokens[0][..tokens[0].len()-1].parse()?;

        let nums: Vec<usize> = tokens[1..].into_iter().map(|str| {
            let result = str.to_owned().trim().parse::<usize>().unwrap();
            result
        }).collect();
        if possible_config2(target, &nums[1..], nums[0]) {
            sum_possible += target;
        }

        buffer.clear();

    }
    println!("total calibration result: {}", sum_possible);
    
    Ok(())
}