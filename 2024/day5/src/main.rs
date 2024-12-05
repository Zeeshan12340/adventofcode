use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

fn main() {
    let data = fs::read_to_string("./input.txt").unwrap();
    let mut sum1 = 0;
    let mut sum2 = 0;

    let (rules, mut updates): (Vec<&str>, Vec<&str>) = data.lines().partition(|line| line.contains('|'));
    updates.remove(0);

    let rule_pairs: Vec<(i32, i32)> = rules.iter()
                                    .map(|&v| v.split_once('|')
                                    .map(|m| (m.0.parse::<i32>().unwrap(), m.1.parse::<i32>().unwrap())).unwrap()).collect();

    let updates_vec: Vec<Vec<i32>> = updates.iter()
                                    .map(|&u| u.split(",").map(|x| x.parse::<i32>().unwrap()).collect()).collect();

    let mut correct_vecs: Vec<&Vec<i32>> = Vec::new();
    let mut fixed_vecs: Vec<Vec<i32>> = Vec::new();

    for update in updates_vec.iter() {
        if is_correct(update, &rule_pairs) {
            correct_vecs.push(update);
        } else {
            let reordered = reorder(update, &rule_pairs);
            fixed_vecs.push(reordered);
        }
    }

    for i in correct_vecs {
        sum1 += i[i.len()/2];
    }
    println!("Part 1 Sum: {}", sum1);

    for i in fixed_vecs {
        sum2 += i[i.len()/2];
    }
    println!("Part 2 Sum: {}", sum2);
}

fn is_correct(update: &Vec<i32>, rules: &Vec<(i32, i32)>) -> bool {
    for (x, y) in rules.iter() {
        if let (Some(left), Some(right)) = (update.iter().position(|&n| n == *x), update.iter().position(|&n| n == *y)) {
            if left > right {
                return false;
            }
        }
    }
    true
}

fn reorder(update: &Vec<i32>, rules: &Vec<(i32, i32)>) -> Vec<i32> {
    let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut indegree: HashMap<i32, usize> = HashMap::new();

    // Initialize graph and indegree
    for &page in update {
        graph.entry(page).or_default();
        indegree.entry(page).or_insert(0);
    }

    // Build graph based on rules
    for &(x, y) in rules.iter() {
        if update.contains(&x) && update.contains(&y) {
            graph.entry(x).or_default().insert(y);
            *indegree.entry(y).or_insert(0) += 1;
        }
    }

    // Topological sort (Kahn's algorithm)
    let mut queue: VecDeque<i32> = indegree.iter()
                                          .filter(|&(_, &deg)| deg == 0)
                                          .map(|(&node, _)| node)
                                          .collect();
    let mut sorted = Vec::new();

    while let Some(node) = queue.pop_front() {
        sorted.push(node);
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                let entry = indegree.entry(neighbor).or_default();
                *entry -= 1;
                if *entry == 0 {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    sorted
}
