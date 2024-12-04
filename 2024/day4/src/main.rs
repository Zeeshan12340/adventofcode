use std::fs;
mod part1;

fn main() {
    let data = fs::read_to_string("./input.txt").unwrap();
    // convert data into matrix and tranpose to convert vertical->horizontal
    let main_vec: Vec<Vec<String>> = data.lines()
                                .map(|s| s.trim().chars()
                                .map(String::from).collect::<Vec<String>>())
                                .collect::<Vec<Vec<String>>>();
    let mut count = 0;

    for (rindex, row) in main_vec.iter().enumerate() {
        for (cindex, item) in row.iter().enumerate() {
            if item == "A" {
                let (check, xmas) = has_neighbors(&main_vec, rindex, cindex);
                if check { count += xmas; }
            }
        }
    }

    part1::main();
    println!("Part 2: Count of X-MAS: {}", count);
}

fn has_neighbors(matrix: &Vec<Vec<String>>, i: usize, j: usize) -> (bool, usize) {
    let offsets: [[i32; 2]; 9] =
    [[-1, -1], [-1, 0], [-1, 1],
    [0, -1], [0, 0], [0, 1],
    [1, -1], [1, 0], [1, 1]];

    for [a, b] in offsets.iter() {
        let x = i as i32 + a;
        let y = j as i32 + b;
        if x < 0 || y < 0 || x >= matrix.len() as i32 || y >= matrix[0].len() as i32 {
            return (false, 0);
        }
    }

    // check for MAS cross
    let mut crosses = 0;
    if (matrix[i-1][j-1] == "M" && matrix[i+1][j+1] == "S") ||
       (matrix[i-1][j-1] == "S" && matrix[i+1][j+1] == "M") {
        // found one cross line, check if other then true
        if (matrix[i+1][j-1] == "M" && matrix[i-1][j+1] == "S") ||
           (matrix[i+1][j-1] == "S" && matrix[i-1][j+1] == "M") {
            crosses += 1;
        }
    }

    if crosses > 0 {
        return (true, crosses)
    }
    return (false, crosses);
}