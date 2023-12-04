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
            symbol_matches.push(SymbolMatch {
                position: found.start(),
            });
            continue;
        }
    }

    (number_matches, symbol_matches)
}

fn main() {
    let full_text = include_str!("./inputs/part1.txt");

    let line_matches: Vec<(Vec<NumberMatch>, Vec<SymbolMatch>)> =
        full_text.lines().map(|l| get_line_matches(l)).collect();

    let mut numbers_found = Vec::new();

    for (idx, (this_line_numbers, this_line_symbols)) in line_matches.iter().enumerate() {
        let prev_line = if idx == 0 {
            None
        } else {
            line_matches.get(idx - 1)
        };
        let next_line = line_matches.get(idx + 1);

        'outer: for number in this_line_numbers {
            let symbol_min = if number.start == 0 {
                0
            } else {
                number.start - 1
            };

            let symbol_max = number.end + 1;

            for symbol in this_line_symbols {
                if symbol.position >= symbol_min && symbol.position <= symbol_max {
                    numbers_found.push(number.number);
                    continue 'outer;
                }
            }

            if let Some((_, symbols)) = prev_line {
                for symbol in symbols {
                    if symbol.position >= symbol_min && symbol.position <= symbol_max {
                        numbers_found.push(number.number);
                        continue 'outer;
                    }
                }
            }

            if let Some((_, symbols)) = next_line {
                for symbol in symbols {
                    if symbol.position >= symbol_min && symbol.position <= symbol_max {
                        numbers_found.push(number.number);
                        continue 'outer;
                    }
                }
            }
        }
    }

    println!("{}", numbers_found.iter().sum::<u32>())
}
