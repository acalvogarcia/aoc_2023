use std::collections::HashMap;

fn get_result_in_line(line: &str) -> u32 {
    let letters_map = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let digits: Vec<String> = (1..10).map(|d| d.to_string()).collect();

    let mut line_digits: Vec<(usize, u32)> = vec![];

    for digit in &digits {
        let mut matches = line
            .match_indices(digit)
            .map(|(position, digit)| (position, digit.parse::<u32>().unwrap()))
            .collect::<Vec<_>>();
        line_digits.append(&mut matches);
    }

    for key in letters_map.keys() {
        let mut matches = line
            .match_indices(key)
            .map(|(position, digit)| (position, *letters_map.get(digit).unwrap()))
            .collect::<Vec<_>>();
        line_digits.append(&mut matches);
    }

    line_digits.sort();

    line_digits.first().unwrap().1 * 10 + line_digits.last().unwrap().1
}

fn main() {
    let full_text = include_str!("./inputs/part2.txt");

    let mut result = 0;

    for line in full_text.lines() {
        result += get_result_in_line(line);
    }

    println!("{result}")
}
