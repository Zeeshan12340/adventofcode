use std::fs;
use diagonal::{diagonal_pos_pos, diagonal_pos_neg};

pub fn main() {
    let data = fs::read_to_string("./input.txt").unwrap();
    // convert data into matrix and tranpose to convert vertical->horizontal
    let main_vec: Vec<Vec<String>> = data.lines()
                                .map(|s| s.trim().chars()
                                .map(String::from).collect::<Vec<String>>())
                                .collect::<Vec<Vec<String>>>();
    let transpose_vec: Vec<Vec<String>> = transpose(&main_vec);
    let vert_data = transpose_vec.iter().map(|item| item.concat())
                                .collect::<Vec<_>>().join("\n");
    let diagonals_vec: Vec<Vec<&String>> = diagonal_pos_pos(&main_vec);
    let diag_data = diagonals_vec.into_iter()
                                .map(|item| item.into_iter().map(String::as_str).collect::<Vec<_>>().concat())
                                .collect::<Vec<_>>().join("\n");
    let neg_diagonals_vec = diagonal_pos_neg(&main_vec);
    let neg_diag_data = neg_diagonals_vec.into_iter()
                                .map(|item| item.into_iter().map(String::as_str).collect::<Vec<_>>().concat())
                                .collect::<Vec<_>>().join("\n");

    // search words in horizontal order(simplest)
    let horizontal: usize = data.match_indices("XMAS").count();
    // println!("horizontal {:?}", horizontal);

    // search words in vertical order
    let vertical: usize = vert_data.match_indices("XMAS").count();
    // println!("vertical {:?}", vertical);

    // search words in diagonal order
    let diagonal: usize = diag_data.match_indices("XMAS").count();
    // println!("diagonal {}", diagonal);

    // search words in negative diagonal order
    let neg_diagonal: usize = neg_diag_data.match_indices("XMAS").count();
    // println!("neg_diagonal {}", neg_diagonal);

    // reverses
    let rev_data = data.chars().rev().collect::<String>();
    let rev_horizontal = rev_data.match_indices("XMAS").count();
    // println!("rev_horizontal {:?}", rev_horizontal);

    let rev_vert_data = vert_data.chars().rev().collect::<String>();
    let rev_vertical = rev_vert_data.match_indices("XMAS").count();
    // println!("rev_vertical {:?}", rev_vertical);

    let rev_diag_data = diag_data.chars().rev().collect::<String>();
    let rev_diagonal = rev_diag_data.match_indices("XMAS").count();
    // println!("rev_diagonal {:?}", rev_diagonal);

    let rev_neg_diag_data = neg_diag_data.chars().rev().collect::<String>();
    let rev_neg_diagonal = rev_neg_diag_data.match_indices("XMAS").count();
    // println!("rev_neg_diagonal {:?}", rev_neg_diagonal);

    println!("Part 1: Count of XMAS: {}", horizontal + vertical + diagonal + neg_diagonal + rev_horizontal + rev_vertical + rev_diagonal + rev_neg_diagonal);
}

fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}