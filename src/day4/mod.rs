use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

const CHARS: [char; 4] = ['X', 'M', 'A', 'S'];

/**
 * I'm really lazy so I'm doing this as 8 hardcoded loops instead of a single loop which considers the offsets
 */
fn count_at_pos(
    lines: &Vec<Vec<char>>,
    row: i32,
    col: i32,
    row_limit: usize,
    col_limit: usize,
) -> u32 {
    let mut total = 0;
    let mut valid = 1;
    let mut curr_row = row;
    let mut curr_col = col;

    // count right
    for c in CHARS {
        if curr_row < 0
            || curr_col < 0
            || curr_row >= row_limit as i32
            || curr_col >= col_limit as i32
            || lines[curr_row as usize][curr_col as usize] != c
        {
            valid = 0;
            break;
        }
        curr_row += 1;
    }
    total += valid;

    // count down
    valid = 1;
    curr_row = row;
    curr_col = col;
    for c in CHARS {
        if curr_row < 0
            || curr_col < 0
            || curr_row >= row_limit as i32
            || curr_col >= col_limit as i32
            || lines[curr_row as usize][curr_col as usize] != c
        {
            valid = 0;
            break;
        }
        curr_col += 1;
    }
    total += valid;

    // count left
    valid = 1;
    curr_row = row;
    curr_col = col;
    for c in CHARS {
        if curr_row < 0
            || curr_col < 0
            || curr_row >= row_limit as i32
            || curr_col >= col_limit as i32
            || lines[curr_row as usize][curr_col as usize] != c
        {
            valid = 0;
            break;
        }
        curr_col -= 1;
    }
    total += valid;

    // count up
    valid = 1;
    curr_row = row;
    curr_col = col;
    for c in CHARS {
        if curr_row < 0
            || curr_col < 0
            || curr_row >= row_limit as i32
            || curr_col >= col_limit as i32
            || lines[curr_row as usize][curr_col as usize] != c
        {
            valid = 0;
            break;
        }
        curr_row -= 1;
    }
    total += valid;

    // count right up
    valid = 1;
    curr_row = row;
    curr_col = col;
    for c in CHARS {
        if curr_row < 0
            || curr_col < 0
            || curr_row >= row_limit as i32
            || curr_col >= col_limit as i32
            || lines[curr_row as usize][curr_col as usize] != c
        {
            valid = 0;
            break;
        }
        curr_row -= 1;
        curr_col += 1;
    }
    total += valid;

    // count right down
    valid = 1;
    curr_row = row;
    curr_col = col;
    for c in CHARS {
        if curr_row < 0
            || curr_col < 0
            || curr_row >= row_limit as i32
            || curr_col >= col_limit as i32
            || lines[curr_row as usize][curr_col as usize] != c
        {
            valid = 0;
            break;
        }
        curr_row += 1;
        curr_col += 1;
    }
    total += valid;

    // count left up
    valid = 1;
    curr_row = row;
    curr_col = col;
    for c in CHARS {
        if curr_row < 0
            || curr_col < 0
            || curr_row >= row_limit as i32
            || curr_col >= col_limit as i32
            || lines[curr_row as usize][curr_col as usize] != c
        {
            valid = 0;
            break;
        }
        curr_row -= 1;
        curr_col -= 1;
    }
    total += valid;

    // count left down
    valid = 1;
    curr_row = row;
    curr_col = col;
    for c in CHARS {
        if curr_row < 0
            || curr_col < 0
            || curr_row >= row_limit as i32
            || curr_col >= col_limit as i32
            || lines[curr_row as usize][curr_col as usize] != c
        {
            valid = 0;
            break;
        }
        curr_row += 1;
        curr_col -= 1;
    }
    total += valid;

    total
}

pub fn count_xmas_count() -> Result<(), Box<dyn Error>> {
    let input = File::open("data/day4/input.txt")?;
    let reader = BufReader::new(input);
    let mut lines: Vec<Vec<char>> = vec![];
    for line in reader.lines() {
        let line = line?;
        lines.push(line.as_bytes().into_iter().map(|b| *b as char).collect());
    }
    let mut count: u32 = 0;
    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            count += count_at_pos(&lines, i as i32, j as i32, lines.len(), lines[i].len());
        }
    }
    println!("Counted XMAS {:?} times", count);

    Ok(())
}

fn count_mas_x(
    lines: &Vec<Vec<char>>,
    row: i32,
    col: i32,
    row_limit: usize,
    col_limit: usize,
) -> u32 {
    if row - 1 < 0
        || col - 1 < 0
        || row + 1 >= row_limit as i32
        || col + 1 >= col_limit as i32
        || lines[row as usize][col as usize] != 'A'
    {
        return 0;
    }

    // both M on top
    if lines[(row - 1) as usize][(col - 1) as usize] == 'M'
        && lines[(row - 1) as usize][(col + 1) as usize] == 'M'
        && lines[(row + 1) as usize][(col - 1) as usize] == 'S'
        && lines[(row + 1) as usize][(col + 1) as usize] == 'S'
    {
        return 1;
    }

    // both M on left side
    if lines[(row - 1) as usize][(col - 1) as usize] == 'M'
        && lines[(row - 1) as usize][(col + 1) as usize] == 'S'
        && lines[(row + 1) as usize][(col - 1) as usize] == 'M'
        && lines[(row + 1) as usize][(col + 1) as usize] == 'S'
    {
        return 1;
    }

    // both M on right side
    if lines[(row - 1) as usize][(col - 1) as usize] == 'S'
        && lines[(row - 1) as usize][(col + 1) as usize] == 'M'
        && lines[(row + 1) as usize][(col - 1) as usize] == 'S'
        && lines[(row + 1) as usize][(col + 1) as usize] == 'M'
    {
        return 1;
    }

    // both M on bottom
    if lines[(row - 1) as usize][(col - 1) as usize] == 'S'
        && lines[(row - 1) as usize][(col + 1) as usize] == 'S'
        && lines[(row + 1) as usize][(col - 1) as usize] == 'M'
        && lines[(row + 1) as usize][(col + 1) as usize] == 'M'
    {
        return 1;
    }

    return 0;
}

pub fn count_masx_all() -> Result<(), Box<dyn Error>> {
    let input = File::open("data/day4/input.txt")?;
    let reader = BufReader::new(input);
    let mut lines: Vec<Vec<char>> = vec![];
    for line in reader.lines() {
        let line = line?;
        lines.push(line.as_bytes().into_iter().map(|b| *b as char).collect());
    }
    let mut count: u32 = 0;
    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            count += count_mas_x(&lines, i as i32, j as i32, lines.len(), lines[i].len());
        }
    }
    println!("Counted MAS-X {:?} times", count);

    Ok(())
}
