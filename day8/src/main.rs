use std::collections::HashMap;

fn read_input_antennas_by_frequency() -> HashMap<String, Vec<Vec<String>>> {
    let input = include_str!("../input.txt");
    let mut antennas_by_freq = HashMap::new();
    for line in input.lines() {
        for ch in line.chars() {
            if ch != '.' {
                antennas_by_freq.entry(ch.to_string()).or_insert(Vec::new());
            }
        }
    }
    for line in input.lines() {
        let mut frequencies_in_line = HashMap::new();
        for ch in line.chars() {
            if ch != '.' {
                let cleaned_line: String = line
                    .chars()
                    .map(|c| if c == ch || c == '.' { c } else { '.' })
                    .collect();
                antennas_by_freq.get_mut(&ch.to_string()).unwrap().push(
                    cleaned_line
                        .split_whitespace()
                        .map(|s| s.to_string())
                        .collect(),
                );
                frequencies_in_line.insert(ch, true);
            }
        }
        for freq in antennas_by_freq.clone().keys() {
            if !frequencies_in_line.contains_key(&freq.chars().next().unwrap()) {
                let empty_line: String = line.chars().map(|_| '.').collect();
                antennas_by_freq.get_mut(freq).unwrap().push(
                    empty_line
                        .split_whitespace()
                        .map(|s| s.to_string())
                        .collect(),
                );
            }
        }
    }
    antennas_by_freq
}

fn print_antennas(freq: String, antennas: Vec<Vec<String>>) {
    println!("Frequency: {}", freq);
    for antenna in antennas {
        for line in antenna {
            println!("{}", line);
        }
    }
}

fn part_one() -> i64 {
    let antennas = read_input_antennas_by_frequency();
    for (freq, antennas) in antennas.iter() {
        print_antennas(freq.to_string(), antennas.clone());
    }
    0
}

fn part_two() -> i64 {
    0
}

fn main() {
    println!("----- AOC 2024 DAY 8 -----");
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}
