use std::{collections::{HashMap, HashSet}, error::Error, fs::File, io::{BufRead, BufReader}};

fn dfs(graph: &HashMap<u32, Vec<u32>>, visited: &mut HashSet<u32>, u: u32) {
    visited.insert(u);

    if let Some(neighbors) = graph.get(&u) {
        for v in neighbors {
            if !visited.contains(v) {
                dfs(graph, visited, *v);
            }
        }
    }
}

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
                // perform a graph traversal, ensure that we do not see nums[i] -> ... nums[j] somewhere
                let mut visited_from_curr = HashSet::new();
                dfs(&rules, &mut visited_from_curr, ordering[i]);
                for j in 0..i {
                    if visited_from_curr.contains(&ordering[j]) {
                        return false;
                    }
                }
            }
            true
        })
        .map(|ordering| {
            ordering[ordering.len()/2]
        })
        .sum();


    println!("Sum of middle values: {:?}", sum_middle_page);
    Ok(())
}