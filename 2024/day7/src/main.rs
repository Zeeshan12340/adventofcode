use std::fs;

fn main() {
    let data = fs::read_to_string("./input.txt").expect("Unable to read file");
    let mut sum1 = 0;
    let mut sum2 = 0;

    for equation in data.lines() {
        let parts: Vec<&str> = equation.split(":").collect();
        let answer = parts[0].parse::<u64>().unwrap();
        let numbers: Vec<u64> = parts[1]
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        // see if numbers can be added or multipled to get answer
        if find_answer(&numbers, answer, false) {
            sum1 += answer;
        }
        if find_answer(&numbers, answer, true) {
            sum2 += answer;
        }
    }
    println!("Part 1 sum: {}", sum1);
    println!("Part 2 sum: {}", sum2);
}

fn find_answer(numbers: &Vec<u64>, answer: u64, part2: bool   ) -> bool {
    // base case
    if numbers.len() == 1 {
        return numbers[0] == answer;
    }

    // try adding and multiplying numbers
    let mut new_numbers = numbers.clone();
    let first = new_numbers.remove(0);
    let second = new_numbers.remove(0);

    // addition recurse case
    new_numbers.insert(0, first + second);
    if find_answer(&new_numbers, answer, part2) {
        return true;
    }
    new_numbers.remove(0);

    // multiplication recurse case
    new_numbers.insert(0,first * second);
    if find_answer(&new_numbers, answer, part2) {
        return true;
    }
    new_numbers.remove(0);

    if part2 {
        // concat recurse case
        let mut joined = first.to_string();
        joined.push_str(&second.to_string());
        new_numbers.insert(0,joined.parse::<u64>().unwrap());
        if find_answer(&new_numbers, answer, part2) {
            return true;
        }
        new_numbers.remove(0);
    }

    false
}