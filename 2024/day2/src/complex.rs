use std::fs;
use itertools::Itertools;
use std::collections::HashSet;

// answer is between 503 and 521 (!512, !515, !516)
fn main() {
    // read input
    let data = fs::read_to_string("./input.txt").unwrap();
    let mut safe = 0;

    for line in data.lines() {
        let mut arr: Vec<i32> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        let mut reduced = false;

        // check if there are duplicates
        if !arr.iter().all_unique() {
            let mut skip = false;
            let mut multiple = 0;
            for item in arr.iter() {
                let count = arr.iter().filter(|&n| *n == *item).count();
                // if number appears thrice, then skip
                if count > 2 { skip = true };
                // if there's two different duplicates
                if count == 2 { multiple += 1 };
            }
            if skip {
                // println!("Skipped because 3 {:?}", arr);
                continue;
            } else if multiple > 2 {
                // println!("Multiple duplicates {:?}", arr);
                continue;
            } else {
                let previous = arr.len();
                arr = remove_duplicates(arr);
                let next = arr.len();
                if next < previous {
                    reduced = true;
                }
                // println!("Single duplicates {:?}", arr);
            }
        }

        // check if array is neither ascending or descending
        let order = in_order(arr.clone());
        if !order && !reduced {
            let mut skip = true;
            for i in 0..arr.len() {
                let mut other = arr.clone();
                other.remove(i);
                if in_order(other.clone()) && other.windows(2).all(|w| (w[0] - w[1]).abs() <= 3) {
                    println!("removing one item from {:?} fixed: {:?}", arr, other);
                    arr = other;
                    skip = false;
                    break;
                }
            }
            if skip { continue; }
        }

        // check difference between adjacent numbers
        if arr.windows(2).all(|w| (w[0] - w[1]).abs() <= 3) {
            println!("arr: {:?}", arr);
            safe += 1;
        };
    }
    println!("{:?}", safe);
}

fn remove_duplicates(mine: Vec<i32>) -> Vec<i32> {
    let mut seen = HashSet::new();
    let mut s = mine.clone();
    let mut duplicate: i32 = 0;

    s.retain(|c| {
        let is_first = !seen.contains(c);
        seen.insert(c.clone());
        if !is_first {
            duplicate = c.clone();
        }
        is_first
    });
    let is_ascending = s.windows(2).all(|w| w[0] <= w[1]);
    let is_descending = s.windows(2).all(|w| w[0] >= w[1]);
    if is_ascending || is_descending {
        println!("fixed by removing duplicate: {:?}", s);
        return s;
    }

    let mut p = mine.clone();
    p.retain(|c| {
        let mut is_first = true;
        if *c == duplicate {
            is_first = false;
            duplicate = -1;
        }
        is_first
    });

    return p;
}
fn in_order(mine: Vec<i32>) -> bool {
    let is_ascending = mine.windows(2).all(|w| w[0] <= w[1]);
    let is_descending = mine.windows(2).all(|w| w[0] >= w[1]);

    if is_ascending || is_descending {
        return true;
    }
    return false;
}