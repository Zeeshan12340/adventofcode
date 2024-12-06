use std::fs;
mod utils;

// 4695 < ans < 5128
fn main() {
    let data = fs::read_to_string("./input.txt").expect("Unable to read file");
    let map: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();

    let distinct_positions = search(&map);
    println!("Part 1 Positions count: {}", distinct_positions.len());
}

fn search(map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut guard: (usize, usize) = (0, 0);
    let mut distinct_positions: Vec<(usize, usize)> = Vec::new();

    for (row, line) in map.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            if *c == '^' {
                guard = (row, col);
            }
        }
    }

    // traverse guard to top of the map and count the positions
    distinct_positions.push(guard);
    loop {
        if utils::upward(&map, &mut guard, &mut distinct_positions) {
            break;
        }
        if utils::rightward(&map, &mut guard, &mut distinct_positions) {
            break;
        }
        if utils::downward(&map, &mut guard, &mut distinct_positions)  {
            break;
        }
        if utils::leftward(&map, &mut guard, &mut distinct_positions)  {
            break;
        }
    }
    distinct_positions
}