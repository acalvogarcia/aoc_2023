use regex::Regex;

#[derive(Debug, Clone)]
struct SymbolMatch {
    pub position: usize,
}

#[derive(Debug, Clone)]
struct NumberMatch {
    pub start: usize,
    pub end: usize,
    pub number: u32,
}

fn get_line_matches(line: &str) -> (Vec<NumberMatch>, Vec<SymbolMatch>) {
    let re = Regex::new(r"([[:digit:]]+)|([^.^[[:digit:]]])").unwrap();

    let mut number_matches: Vec<NumberMatch> = Vec::new();
    let mut symbol_matches: Vec<SymbolMatch> = Vec::new();

    for cap in re.captures_iter(line) {
        if let Some(found) = &cap.get(1) {
            number_matches.push(NumberMatch {
                start: found.start(),
                end: found.end() - 1,
                number: found.as_str().parse().unwrap(),
            });
            continue;
        }

        if let Some(found) = &cap.get(2) {
            if found.as_str() == "*" {
                symbol_matches.push(SymbolMatch {
                    position: found.start(),
                });
            }
            continue;
        }
    }

    (number_matches, symbol_matches)
}

fn main() {
    let full_text = include_str!("./inputs/part1.txt");

    let line_matches: Vec<(Vec<NumberMatch>, Vec<SymbolMatch>)> =
        full_text.lines().map(|l| get_line_matches(l)).collect();

    let mut gear_ratios = Vec::new();

    for (idx, (this_line_numbers, this_line_symbols)) in line_matches.iter().enumerate() {
        let prev_line = if idx == 0 {
            None
        } else {
            line_matches.get(idx - 1)
        };
        let next_line = line_matches.get(idx + 1);

        for symbol in this_line_symbols {
            let mut adjacent_numbers = Vec::new();

            let area_start = if symbol.position == 0 {
                0
            } else {
                symbol.position - 1
            };
            let area_end = symbol.position + 1;

            for number in this_line_numbers {
                if (area_start <= number.start && area_end >= number.start)
                    || (area_start <= number.end && area_end >= number.end)
                {
                    adjacent_numbers.push(number.number);
                }
            }

            if let Some((numbers, _)) = prev_line {
                for number in numbers {
                    if (area_start <= number.start && area_end >= number.start)
                        || (area_start <= number.end && area_end >= number.end)
                    {
                        adjacent_numbers.push(number.number);
                    }
                }
            };

            if let Some((numbers, _)) = next_line {
                for number in numbers {
                    if (area_start <= number.start && area_end >= number.start)
                        || (area_start <= number.end && area_end >= number.end)
                    {
                        adjacent_numbers.push(number.number);
                    }
                }
            };

            if adjacent_numbers.len() == 2 {
                gear_ratios.push(adjacent_numbers[0] * adjacent_numbers[1])
            }
        }
    }

    println!("{}", gear_ratios.iter().sum::<u32>())
}
