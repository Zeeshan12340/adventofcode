fn check_neighbours_for_symbol(i: usize, j: usize, matrix: &Vec<Vec<char>>) -> bool {
    let offsets: [[i32; 2]; 9] = 
    [[-1, -1], [-1, 0], [-1, 1],
    [0, -1], [0, 0], [0, 1],
    [1, -1], [1, 0], [1, 1]];
    println!("Number: {}", matrix[i][j]);
    for [a, b] in offsets.iter() {
        let x = i as i32 + a;
        let y = j as i32 + b;
        if x >= 0 && y >= 0 && x < matrix.len() as i32 && y < matrix[0].len() as i32 {
            if matrix[x as usize][y as usize] != '.' && !matrix[x as usize][y as usize].is_digit(10) {
                println!("found symbol at x:{} y:{}, n:{}", x, y, matrix[x as usize][y as usize]);
                return true;
            }
        }
    }
    return false;
}

fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    let matrix: Vec<Vec<char>> = data
        .lines()
        .map(|line| line.chars().map(|s| s).collect())
        .collect();

    let mut current_number;
    let mut good = false;
    let mut result = 0;
    for (i, row) in matrix.iter().enumerate() {
        current_number = String::new();
        println!("---- Row: {} ------------", i);
        for (j, &col) in row.iter().enumerate() {        
            if col.is_digit(10) {
                current_number.push(col);
                if check_neighbours_for_symbol(i, j, &matrix) {
                    good = true; 
                }
            } else if !current_number.is_empty() {
                    println!("----- next number: {} ----", current_number);
                    if let Ok(number) = current_number.parse::<u32>() {
                        if good {
                            result += number;
                            good = false;
                        }
                    }
                    current_number.clear();
            }
        }
        // if the number is the last in the row, the above if else wont trigger
        if !current_number.is_empty() {
            println!("----- next number: {} ----", current_number);
            if let Ok(number) = current_number.parse::<u32>() {
                if good {
                    result += number;
                    good = false;
                }
            }
            current_number.clear();
        }
    }
    println!("Result: {}", result);
}
