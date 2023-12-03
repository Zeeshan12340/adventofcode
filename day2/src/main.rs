fn main() {
    let games = std::fs::read_to_string("./input.txt").expect("Failed to read input file");
    
    let (red, green, blue) = (12,13,14);
    let mut result = 0;
    for game in games.lines() {
        let mut parts = game.split(":");
        let id = parts.next().unwrap().split_whitespace().last().unwrap().parse::<u32>().expect("failed to parse game id");

        let cube_sets = parts.next().unwrap().split(";").map(|cubes| {
            cubes.split(",").map(|cube| {
                let mut parts = cube.split_whitespace();
                let count = parts.next().unwrap().trim().parse::<u32>().unwrap();
                let color = parts.next().unwrap().trim();
                (count, color)
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>();

        let mut possible = true;
        for cube_set in cube_sets.iter() {
            let mut red_count = 0;
            let mut green_count = 0;
            let mut blue_count = 0;
            for (count, color) in cube_set.iter() {
                match *color {
                    "red" => red_count += count,
                    "green" => green_count += count,
                    "blue" => blue_count += count,
                    _ => panic!("Unknown color {}", color)
                }
            }
            if red_count > red || green_count > green || blue_count > blue {
                println!("Game: {} and cubes sets {:?} not possible", id, cube_sets);
                possible = false;
                break;
            }
        }
        if possible {
            result += id;
        }
    }
    println!("Result: {}", result);
}
