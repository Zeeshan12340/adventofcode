use std::fs;

fn main() {
    // read input to file
    let contents = fs::read_to_string("./input.txt")
        .expect("Something went wrong reading the file");
    
    let mut first_column: Vec<i32> = Vec::new();
    let mut second_column: Vec<i32> = Vec::new();
    let mut difference_column: Vec<i32> = Vec::new();
    let mut similarity_column: Vec<i32> = Vec::new();

    // split input into lines and then split each line into first and second column
    let lines: Vec<&str> = contents.lines().collect();
    for line in lines {
        let parts: Vec<&str> = line.split("   ").collect();
        first_column.push(parts[0].parse::<i32>().unwrap());
        second_column.push(parts[1].parse::<i32>().unwrap());
    }

    // sort both vectors
    first_column.sort();
    second_column.sort();

    // subtract values from both columns
    for i in 0..first_column.len() {
        difference_column.push((first_column[i] - second_column[i]).abs());
    }

    // sum up the difference vector
    let sum = difference_column.iter().fold(0, |acc, x| acc + x);
    println!("Difference Score: {}", sum);

    // PART TWO
    // calculate similarity between the two columns
    for item in first_column.iter() {
        if !second_column.contains(item) {
            similarity_column.push(0);
        } else {
            let n: i32 = second_column.iter().filter(|second| *second == item).count().try_into().unwrap();
            similarity_column.push(item * n);
        }
    }

    let sum: i32 = similarity_column.iter().sum();
    println!("Similarity Score: {}", sum);
}