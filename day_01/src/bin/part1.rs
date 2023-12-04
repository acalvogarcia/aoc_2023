fn main() {
    let full_text = include_str!("./inputs/part1.txt");

    let result = full_text
        // .split("\n")
        .lines()
        .map(|line| {
            line.split("")
                .filter_map(|c| c.parse::<u32>().ok())
                .collect::<Vec<_>>()
        })
        .map(|line| line.first().unwrap() * 10 + line.last().unwrap())
        .collect::<Vec<_>>();
    // .map(|mut line| format!("{}{}", line.next().unwrap(), line.last().unwrap()))
    // .map(|line| line.parse::<u32>().unwrap())
    // .sum();

    println!("{:?}", result.iter().sum::<u32>())
}
