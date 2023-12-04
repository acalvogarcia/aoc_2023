use std::collections::HashSet;

use regex::Regex;

fn get_line_points(line: &str) -> u32 {
    let (_, results_part) = line.split_once(": ").unwrap();

    let (winning_numbers_part, your_numbers_part) = results_part.split_once(" | ").unwrap();

    let number_re = Regex::new("([[:digit:]]+)").unwrap();

    let winning_numbers: HashSet<u32> = number_re
        .captures_iter(winning_numbers_part)
        .map(|c| c.get(1).unwrap().as_str().parse().unwrap())
        .collect();

    let your_numbers: HashSet<u32> = number_re
        .captures_iter(your_numbers_part)
        .map(|c| c.get(1).unwrap().as_str().parse().unwrap())
        .collect();

    let winning_count = your_numbers
        .iter()
        .filter(|n| winning_numbers.contains(n))
        .count();

    if winning_count == 0 {
        0
    } else {
        (2 as u32).pow(winning_count as u32 - 1)
    }
}

fn main() {
    let full_text = include_str!("./inputs/part1.txt");

    let mut result = 0;

    for line in full_text.lines() {
        result += get_line_points(line);
    }

    println!("{result}");
}
