use std::collections::HashMap;
fn read_input_equations() -> HashMap<i32, Vec<i32>> {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut equations = HashMap::new();
    for line in input.lines() {
        let mut parts = line.split(":");
        let result = parts.next().unwrap();
        let operands_part = parts.next().unwrap();
        let result = match result.parse::<i32>() {
            Ok(val) => val,
            Err(_) => continue,
        };
        let operands = operands_part
            .split_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();
        equations.insert(result, operands);
    }
    equations
}

fn can_be_resolved_equation(operands: &[i32], target: i32) -> bool {
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

fn part_one() -> i32 {
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

fn part_two() -> i32 {
    0
}

fn main() {
    println!("----- AOC 2024 DAY 7 -----");
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}
