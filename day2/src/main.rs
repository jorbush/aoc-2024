use std::collections::VecDeque;

#[derive(Debug)]
enum Trend {
    Increasing,
    Decreasing,
}

fn read_input_reports() -> Vec<Vec<i32>> {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut reports = Vec::new();
    for line in input.lines() {
        let levels = line.split_whitespace();
        let mut report = Vec::new();
        for level in levels {
            report.push(level.parse().unwrap());
        }
        reports.push(report);
    }
    reports
}

fn is_safe(report: Vec<i32>) -> bool {
    let mut levels = VecDeque::from(report.clone());
    let mut last_level = levels.pop_front().unwrap();
    let trend = if &last_level < levels.front().unwrap() {
        Trend::Increasing
    } else {
        Trend::Decreasing
    };
    for level in levels {
        if level == last_level {
            return false;
        }
        match trend {
            Trend::Increasing => {
                if level < last_level || (level - last_level).abs() > 3 {
                    return false;
                }
            }
            Trend::Decreasing => {
                if level > last_level || (level - last_level).abs() > 3 {
                    return false;
                }
            }
        }
        last_level = level;
    }
    true
}

fn is_safe_tolerant(report: Vec<i32>) -> bool {
    let mut levels = VecDeque::from(report.clone());
    let mut last_level = levels.pop_front().unwrap();
    let trend = if &last_level < levels.front().unwrap() {
        Trend::Increasing
    } else {
        Trend::Decreasing
    };
    let mut fail_one_level = false;
    for level in levels {
        match trend {
            Trend::Increasing => {
                let diff = (level - last_level).abs();
                if level < last_level || diff > 3 || diff < 1 {
                    if fail_one_level {
                        return false;
                    }
                    fail_one_level = true;
                    continue;
                }
            }
            Trend::Decreasing => {
                let diff = (level - last_level).abs();
                if level > last_level || diff > 3 || diff < 1 {
                    if fail_one_level {
                        return false;
                    }
                    fail_one_level = true;
                    continue;
                }
            }
        }
        last_level = level;
    }
    true
}

fn part_one() -> i32 {
    let reports = read_input_reports();
    let mut safe_reports = 0;
    for report in reports {
        if is_safe(report) {
            safe_reports += 1;
        }
    }
    safe_reports
}

fn part_two() -> i32 {
    let reports = read_input_reports();
    let mut safe_reports = 0;
    for report in reports {
        if is_safe_tolerant(report) {
            safe_reports += 1;
        }
    }
    safe_reports
}

fn main() {
    println!("----- AOC 2024 DAY 2 -----");
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}
