use std::{error::Error, fs::File, io::{BufRead, BufReader, Read}, cmp::{max, min}, collections::HashMap};

pub fn total_list_distance()  -> Result<(), Box<dyn Error + 'static>>{
    let input = File::open("./data/day1/input.txt")?;
    let mut reader = BufReader::new(input);
    // read in line to string buf, add to list
    let mut left_list: Vec<u32> = vec![];
    let mut right_list: Vec<u32> = vec![];
    let mut line_buffer = String::new();
    while let Ok(l) = reader.read_line(&mut line_buffer) {
        if l == 0 {
            break
        }
        let vec: Vec<&str> = line_buffer.split("   ").map(|s| s.trim()).collect();
        left_list.push(vec[0].parse()?);
        right_list.push(vec[1].parse()?);
        line_buffer.clear();
    }
    left_list.sort();
    right_list.sort();
    let answer: u32 = left_list.into_iter().zip(right_list.into_iter()).map(|(left_val, right_val)| {
        max(left_val, right_val) - min(left_val, right_val)
    }).sum();

    println!("Sum of distance is {}", answer);


    Ok(())
}

pub fn similarity_score() -> Result<(), Box<dyn Error>> {
    let input = File::open("./data/day1/input.txt")?;
    let mut reader = BufReader::new(input);
    // read in line to string buf, add to list
    let mut left_list: Vec<u32> = vec![];
    let mut right_counts: HashMap<u32, u32> = HashMap::new();
    let mut line_buffer = String::new();
    while let Ok(l) = reader.read_line(&mut line_buffer) {
        if l == 0 {
            break
        }
        let vec: Vec<&str> = line_buffer.split("   ").map(|s| s.trim()).collect();
        left_list.push(vec[0].parse()?);
        *right_counts.entry(vec[1].parse()?).or_insert(0) += 1;
        line_buffer.clear();
    }
    let similarity_score: u32 = left_list.into_iter().map(|num| right_counts.get(&num).unwrap_or(&0) * num).sum();
    println!("Similarity score is {}", similarity_score);

    Ok(())
}