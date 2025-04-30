use std::{collections::{HashMap, HashSet}, error::Error, fs::File, io::{BufRead, BufReader}};

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Direction {
    pub fn offset(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1)
        }
    }
    pub fn next_direction(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up
        }
    }
}


pub fn count_distinct_positions() -> Result<(), Box<dyn Error>> {
    let input = File::open("data/day6/input.txt")?;
    let reader = BufReader::new(input);
    let mut map: Vec<Vec<char>> = Vec::new();
    
    for line in reader.lines() {
        let line = line?;
        map.push(line.as_bytes().into_iter().map(|b| *b as char).collect());
    }

    let mut pos = (0, 0);
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == '^' {
                pos = (i as i32, j as i32);
                break;
            }
        }
        if (0, 0) != pos {
            break;
        }
    }

    let mut direction = Direction::Up;
    let mut visited: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];
    while pos.0 >= 0 && pos.1 >= 0 && pos.0 < map.len() as i32 && pos.1 < map[0].len() as i32 {
        let (row, col) = pos;
        visited[row as usize][col as usize] = true;

        let mut next_pos = (row + direction.offset().0, col + direction.offset().1);
        if next_pos.0 < 0 || next_pos.1 < 0 || next_pos.0 as usize >= map.len() || next_pos.1 as usize >= map[0].len() {
            break;
        }
        let mut count = 0;
        while map[next_pos.0 as usize][next_pos.1 as usize] == '#' {
            direction = direction.next_direction();
            count += 1;
            next_pos = (row + direction.offset().0, col + direction.offset().1);
            if next_pos.0 < 0 || next_pos.1 < 0 || next_pos.0 as usize >= map.len() || next_pos.1 as usize >= map[0].len() || count >= 4 {
                break;
            }
        }
        pos = next_pos;
    }

    let visited_count: usize = visited.into_iter().map(|row| {
        row.iter().filter(|v| **v).count()
    }).sum();

    println!("Total unique visists: {:?}", visited_count);

    Ok(())
}

fn infinite_loop_exists(map: &Vec<Vec<char>>, mut row: i32, mut col: i32, mut direction: Direction) -> bool {
    // block was placed => see if we get duplicate
    let mut iters = 0;
    const THRESHOLD: usize = 2 * 5461;
    while iters < THRESHOLD && row >= 0 && col >= 0 && (row as usize) < map.len() && (col as usize) < map[0].len() {
        let mut next_pos = (row + direction.offset().0, col + direction.offset().1);
        
        if next_pos.0 < 0 || next_pos.1 < 0 || next_pos.0 as usize >= map.len() || next_pos.1 as usize >= map[0].len() {
            break;
        }
        iters += 1;
        let mut count = 0;
        while map[next_pos.0 as usize][next_pos.1 as usize] == '#' {
            direction = direction.next_direction();
            count += 1;
            next_pos = (row + direction.offset().0, col + direction.offset().1);
            if next_pos.0 < 0 || next_pos.1 < 0 || next_pos.0 as usize >= map.len() || next_pos.1 as usize >= map[0].len() || count >= 4 {
                break;
            }
        }
        (row, col) = next_pos;
    }

    iters >= THRESHOLD
}

pub fn count_distinct_block_positions() -> Result<(), Box<dyn Error>> {
    let input = File::open("data/day6/input.txt")?;
    let reader = BufReader::new(input);
    let mut map: Vec<Vec<char>> = Vec::new();
    
    for line in reader.lines() {
        let line = line?;
        map.push(line.as_bytes().into_iter().map(|b| *b as char).collect());
    }

    let mut pos = (0, 0);
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == '^' {
                pos = (i as i32, j as i32);
                break;
            }
        }
        if (0, 0) != pos {
            break;
        }
    }

    let mut direction = Direction::Up;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    while pos.0 >= 0 && pos.1 >= 0 && pos.0 < map.len() as i32 && pos.1 < map[0].len() as i32 {
        let (row, col) = pos;
        
        let mut next_pos = (row + direction.offset().0, col + direction.offset().1);
        if next_pos.0 < 0 || next_pos.1 < 0 || next_pos.0 as usize >= map.len() || next_pos.1 as usize >= map[0].len() {
            break;
        }
        let old_item = map[next_pos.0 as usize][next_pos.1 as usize];
        if old_item == '.' {
            map[next_pos.0 as usize][next_pos.1 as usize] = '#';
            if !visited.contains(&next_pos) && infinite_loop_exists(&map, row, col, direction.next_direction()) {
                visited.insert(next_pos);
            }
            map[next_pos.0 as usize][next_pos.1 as usize] = old_item;
        }
        

        let mut count: u8 = 0;
        while map[next_pos.0 as usize][next_pos.1 as usize] == '#' {
            direction = direction.next_direction();
            count += 1;
            next_pos = (row + direction.offset().0, col + direction.offset().1);
            if next_pos.0 < 0 || next_pos.1 < 0 || next_pos.0 as usize >= map.len() || next_pos.1 as usize >= map[0].len() || count >= 4 {
                break;
            }
        }
        pos = next_pos;
    }

    println!("Total unique block positions: {:?}", visited.len());

    Ok(())
}