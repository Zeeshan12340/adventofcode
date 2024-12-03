use std::fs;
use itertools::Itertools;

// answer is between 503 and 521 (!512, !515, !516)
fn main() {
    // read input
    let data = fs::read_to_string("./input.txt").unwrap();
    let mut safe1 = 0;
    let mut safe2 = 0;

    for line in data.lines() {
        let arr: Vec<i32> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();

        if check_conditions(&arr) {
            safe1 += 1
        }

        for i in 0..arr.len() {
            // simply check if removing an element fixes everything
            let mut copy = arr.clone();
            copy.remove(i);
            if check_conditions(&copy) {
                safe2 += 1;
                break;
            }
        }
    }
    println!("part 1 safes: {:?}", safe1);
    println!("part 1 safes: {:?}", safe2);
}

fn check_conditions(arr: &Vec<i32>) -> bool {
    // check if there are duplicates
    if !arr.iter().all_unique() {
        return false;
    }

    // check if in either ascending/descending order
    let is_ascending = arr.windows(2).all(|w| w[0] <= w[1]);
    let is_descending = arr.windows(2).all(|w| w[0] >= w[1]);

    if !is_ascending && !is_descending {
        return false;
    }

    // check difference between adjacent numbers
    if arr.windows(2).all(|w| (w[0] - w[1]).abs() <= 3) {
        return true;
    };
    false
}