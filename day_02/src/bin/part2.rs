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

fn get_line_power(line_data: Vec<Vec<(u32, &str)>>) -> u32 {
    let mut colour_mins: HashMap<&str, u32> = HashMap::new();

    for set in line_data {
        for (amount, colour) in set {
            if let Some(stored_amount) = colour_mins.get(colour) {
                if amount > *stored_amount {
                    colour_mins.insert(colour, amount);
                }
            } else {
                colour_mins.insert(colour, amount);
            }
        }
    }

    colour_mins.values().product()
}

fn main() {
    let full_text = include_str!("./inputs/part1.txt");

    let mut result = 0;

    for line in full_text.lines() {
        let line_data = split_line(line);

        let line_power = get_line_power(line_data.1);

        result += line_power;
    }

    println!("{result}");
}
