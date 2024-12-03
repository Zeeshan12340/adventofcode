use std::fs;

fn main() {
    // read input
    let data = fs::read_to_string("./input.txt").unwrap();
    let arr: Vec<char> = data.chars().collect();
    // get iterables of size 3 from array of chars
    let iter = arr.windows(3);
    let mut enabled = true;
    let mut sum1 = 0;
    let mut sum2 = 0;

    for (index, item) in iter.enumerate() {
        let curr = String::from_iter(item);
        if curr == "don" {
            enabled = false;
        } else if curr == "do(" {
            enabled = true;
        } else if String::from_iter(item) == "mul" {
            // if ( is not front of mul, then skip
            if arr[index+3] != '(' {
                continue;
            }

            let (is_number, first_num) = extract_number(&arr, index+4, ',');
            if !is_number  {
                continue;
            }

            let offset = first_num.to_string().len();
            let (is_number, second_num) = extract_number(&arr, index+5+offset, ')');
            if !is_number  {
                continue;
            }

            sum1 += first_num * second_num;
            if enabled {
                sum2 += first_num * second_num;
            }
        }
    };
    println!("Part 1 sum of multiples: {}", sum1);
    println!("Part 2 sum of multiples: {}", sum2);
}

fn extract_number(arr: &Vec<char>, index: usize, delimiter: char) -> (bool, i32) {
    let mut is_first = true;
    let mut num: Vec<char> = Vec::new();

    for i in index..index+4 {
        if is_first && !arr[i].is_numeric() {
            return (false, -1);
        }
        if !is_first && !arr[i].is_numeric() && arr[i] != delimiter {
            return (false, -1);
        }
        if arr[i] == delimiter { break; }
        num.push(arr[i]);
        is_first = false;
    }
    let extracted = String::from_iter(num).parse::<i32>().unwrap();
    return (true, extracted);
}
