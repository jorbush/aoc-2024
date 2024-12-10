use std::{collections::HashMap, process::exit};
fn read_input_equations() -> HashMap<i64, Vec<i64>> {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut equations = HashMap::new();
    for line in input.lines() {
        let mut parts = line.split(":");
        let result = parts.next().unwrap();
        let operands_part = parts.next().unwrap();
        let result = match result.parse::<i64>() {
            Ok(val) => val,
            Err(err) => {
                println!("Error parsing result: {}", result);
                println!("Line: {}", line);
                println!("Error: {}", err);
                exit(0);
            }
        };
        let operands = operands_part
            .split_whitespace()
            .filter_map(|x| x.parse::<i64>().ok())
            .collect();
        equations.insert(result, operands);
    }
    equations
}

fn can_be_resolved_equation(operands: &[i64], target: i64) -> bool {
    if target < 0 {
        return false;
    }
    if operands.len() == 1 {
        return operands[0] == target;
    }
    for i in 0..operands.len() {
        let mut remaining = operands.to_vec();
        let current = remaining.remove(i);
        if can_be_resolved_equation(&remaining, target - current)
            || can_be_resolved_equation(&remaining, target / current)
        {
            return true;
        }
    }
    false
}

fn part_one() -> i64 {
    let equations = read_input_equations();
    let mut sum_resolved = 0;
    for (result, operands) in equations.iter() {
        println!("Resolving: {} {:?}", result, operands);
        if can_be_resolved_equation(operands, *result) {
            sum_resolved += result;
        }
    }
    sum_resolved
}

fn part_two() -> i64 {
    0
}

fn main() {
    println!("----- AOC 2024 DAY 7 -----");
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}
