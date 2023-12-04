use std::collections::HashMap;

fn get_game_id(input: &str) -> u32 {
    input.strip_prefix("Game ").unwrap().parse().unwrap()
}

fn get_game_cubes(input: &str) -> Vec<Vec<(u32, &str)>> {
    input
        .split("; ")
        .map(|set| {
            set.split(", ")
                .map(|cubes| {
                    let split_once = cubes.split_once(" ").unwrap();
                    (split_once.0.parse().unwrap(), split_once.1)
                })
                .collect()
        })
        .collect()
}

fn split_line(line: &str) -> (u32, Vec<Vec<(u32, &str)>>) {
    let (game_id_part, game_info_part) = line.split_once(": ").unwrap();

    let game_id = get_game_id(game_id_part);
    let game_cubes = get_game_cubes(game_info_part);

    (game_id, game_cubes)
}

fn check_line(line_data: Vec<Vec<(u32, &str)>>) -> bool {
    let max_cubes = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    for set in line_data {
        for (amount, colour) in set {
            if amount > max_cubes[colour] {
                return false;
            }
        }
    }

    true
}

fn main() {
    let full_text = include_str!("./inputs/part1.txt");

    let mut result = 0;

    for line in full_text.lines() {
        let line_data = split_line(line);
        if check_line(line_data.1) {
            result += line_data.0
        }
        // println!("{:?}", line_data);
        // println!("{}", check_line(line_data.1));
    }

    println!("{result}")
}
