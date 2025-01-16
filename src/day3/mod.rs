use regex::{Match, Regex};
use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
};

pub fn sum_uncorrupted_mults() -> Result<(), Box<dyn Error>> {
    let input = File::open("data/day3/input.txt")?;
    let mut reader = BufReader::new(input);
    let mut contents = Vec::new();
    reader.read_to_end(&mut contents)?;
    let data = String::from_utf8(contents)?;

    let pattern = Regex::new("mul[(](0|[1-9][0-9]{0,2}),(0|[1-9][0-9]{0,2})[)]")?;
    let mult_sum: i32 = pattern
        .find_iter(&data)
        .map(|loc| {
            let start = loc.start();
            let end = loc.end();
            let expr = &data[start + 4..end - 1];
            // trust me its fine
            let nums: Vec<i32> = expr.split(",").map(|s| s.parse().unwrap()).collect();
            nums[0] * nums[1]
        })
        .sum();

    println!("Total of multiplications: {:?}", mult_sum);

    Ok(())
}

enum ExprType {
    DoDont { enable: bool },
    Mult,
}
struct MatchedExpr {
    expr_type: ExprType,
    start: usize,
    end: usize,
}

pub fn sum_uncorrupted_do_dont() -> Result<(), Box<dyn Error>> {
    let input = File::open("data/day3/input.txt")?;
    let mut reader = BufReader::new(input);
    let mut contents = Vec::new();
    reader.read_to_end(&mut contents)?;
    let data = String::from_utf8(contents)?;

    let pattern_mults = Regex::new("mul[(](0|[1-9][0-9]{0,2}),(0|[1-9][0-9]{0,2})[)]")?;
    let pattern_do_dont = Regex::new("do[(][)]|don't[(][)]")?;

    let mult_matches = pattern_mults.find_iter(&data).map(|m| MatchedExpr {
        expr_type: ExprType::Mult,
        start: m.start(),
        end: m.end(),
    });
    let do_dont_matches = pattern_do_dont.find_iter(&data).map(|m| {
        let expr_inner_type = &data[m.start()..m.end()] == "do()";
        MatchedExpr {
            expr_type: ExprType::DoDont {
                enable: expr_inner_type,
            },
            start: m.start(),
            end: m.end(),
        }
    });
    let mut all_matches: Vec<MatchedExpr> = mult_matches.chain(do_dont_matches).collect();
    all_matches.sort_by(|m1, m2| {
        if m1.start < m2.start {
            std::cmp::Ordering::Less
        } else if m1.start > m2.start {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });

    let restricted_sum_mult = all_matches
        .into_iter()
        .fold((0, true), |curr, m| match m.expr_type {
            ExprType::DoDont { enable } => (curr.0, enable),
            ExprType::Mult => {
                let expr = &data[m.start + 4..m.end - 1];
                let num_split: Vec<i32> = expr.split(",").map(|s| s.parse().unwrap()).collect();
                if curr.1 {
                    (curr.0 + num_split[0] * num_split[1], curr.1)
                } else {
                    curr
                }
            }
        })
        .0;

    println!(
        "Total of multiplications with do/dont: {:?}",
        restricted_sum_mult
    );

    Ok(())
}
