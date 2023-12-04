use std::collections::{HashMap, HashSet};

use regex::Regex;

fn get_card_number(card_number_part: &str) -> u32 {
    let number_re = Regex::new("([[:digit:]]+)").unwrap();
    number_re
        .captures(card_number_part)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap()
}

fn get_line_matches(results_part: &str) -> u32 {
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

    your_numbers
        .iter()
        .filter(|n| winning_numbers.contains(n))
        .count() as u32
}

fn get_resulting_cards(
    card_number: u32,
    winning_matches: u32,
    previous_results: &HashMap<u32, u32>,
) -> u32 {
    let mut result = 1;

    for i in (card_number + 1)..(card_number + winning_matches + 1) {
        if let Some(sub_result) = previous_results.get(&i) {
            result += sub_result;
        }
    }

    result
}

fn main() {
    let mut cache: HashMap<u32, u32> = HashMap::new();

    let full_text = include_str!("./inputs/part1.txt");

    let mut result = 0;

    for line in full_text.lines().rev() {
        let (card_number_part, results_part) = line.split_once(": ").unwrap();

        let card_number = get_card_number(card_number_part);
        let winning_matches = get_line_matches(results_part);

        let sub_result = get_resulting_cards(card_number, winning_matches, &cache);
        cache.insert(card_number, sub_result);

        result += sub_result;
    }

    println!("{result}");
}
