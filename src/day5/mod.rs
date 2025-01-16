use std::{
    cmp::min,
    collections::{HashMap, HashSet},
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    mem::swap,
};

pub fn sum_middle_correctly_ordered() -> Result<(), Box<dyn Error>> {
    let input = File::open("data/day5/input.txt")?;
    let reader = BufReader::new(input);

    let mut orderings: Vec<Vec<u32>> = Vec::new();
    let mut read_orderings = false;
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            read_orderings = true;
        } else {
            if read_orderings {
                orderings.push(line.split(",").map(|n| n.parse().unwrap()).collect());
            } else {
                let rule: Vec<u32> = line.split("|").map(|n| n.parse().unwrap()).collect();
                rules.entry(rule[0]).or_insert(vec![]).push(rule[1]);
            }
        }
    }
    let sum_middle_page: u32 = orderings
        .into_iter()
        .filter(|ordering| {
            for i in 0..ordering.len() {
                // check that corresponding rule exists for each
                for j in i + 1..ordering.len() {
                    if !rules[&ordering[i]].contains(&ordering[j]) {
                        return false;
                    }
                }
            }
            true
        })
        .map(|ordering| ordering[ordering.len() / 2])
        .sum();

    println!("Sum of middle values: {:?}", sum_middle_page);
    Ok(())
}

pub fn sum_middle_incorrect_fixed() -> Result<(), Box<dyn Error>> {
    let input = File::open("data/day5/input.txt")?;
    let reader = BufReader::new(input);

    let mut orderings: Vec<Vec<u32>> = Vec::new();
    let mut read_orderings = false;
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            read_orderings = true;
        } else {
            if read_orderings {
                orderings.push(line.split(",").map(|n| n.parse().unwrap()).collect());
            } else {
                let rule: Vec<u32> = line.split("|").map(|n| n.parse().unwrap()).collect();
                rules.entry(rule[0]).or_insert(vec![]).push(rule[1]);
            }
        }
    }
    let sum_middle_page: u32 = orderings
        .into_iter()
        .filter_map(|ordering: Vec<u32>| {
            let mut ordering = ordering.clone();
            let mut changed = false;
            for i in (0..ordering.len()).rev() {
                // check that corresponding rule exists for each
                let mut j = ordering.len() - 1;
                while j >= i + 1 {
                    if !rules[&ordering[i]].contains(&ordering[j]) {
                        changed = true;
                        // move to earlier position, as clearly we pass on all of those
                        ordering.insert(min(ordering.len(), j + 1), ordering[i]);
                        ordering.remove(i);
                        break;
                    }
                    j -= 1;
                }
            }
            if changed {
                Some(ordering)
            } else {
                None
            }
        })
        .map(|ordering: Vec<u32>| ordering[ordering.len() / 2])
        .sum();

    println!("Sum of middle fixed values: {:?}", sum_middle_page);
    Ok(())
}
