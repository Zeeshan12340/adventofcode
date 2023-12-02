use std::collections::HashMap;
use reqwest;

fn main() -> Result<(), reqwest::Error> {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    let digit_words: HashMap<&str, u32> = [
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ].iter().cloned().collect();
    let mut found_digits;
    let mut found_digit_words ;
    let mut fdigit: u32 = 0;
    let mut sdigit: u32 = 0;
    let mut result: u32 = 0;

    for line in data.lines() {
        found_digits = HashMap::new();
        found_digit_words = HashMap::new();
        
        for c in line.chars() {
            if c.is_digit(10) {
                line.match_indices(c).for_each(|(offset, _)| {
                    found_digits.insert(offset, c);
                });
            }
        }
        
        for &word in digit_words.keys() {
            if line.contains(word) {
                line.match_indices(word).for_each(|(offset, _)| {
                    found_digit_words.insert(offset, word);
                });
            }
        }

        if found_digits.keys().min().unwrap_or(&1000) <= found_digit_words.keys().min().unwrap_or(&1000) {
            fdigit = found_digits.get(found_digits.keys().min().unwrap()).unwrap().to_digit(10).unwrap() * 10;
        } else if found_digit_words.len() > 0 {
            let word = *found_digit_words.get(found_digit_words.keys().min().unwrap()).unwrap();
            fdigit = *digit_words.get(word).unwrap() * 10;
        }

        if found_digits.keys().max().unwrap_or(&0) >= found_digit_words.keys().max().unwrap_or(&0) {
            sdigit = found_digits.get(found_digits.keys().max().unwrap()).unwrap().to_digit(10).unwrap();
        } else if found_digit_words.len() > 0 {
            let word = *found_digit_words.get(found_digit_words.keys().max().unwrap()).unwrap();
            sdigit = *digit_words.get(word).unwrap();
        }

        println!("{}", fdigit + sdigit);
        result += fdigit + sdigit;
    }
    println!("Result: {}", result);
    Ok(())
}
