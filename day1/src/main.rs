fn read_input_two_lists() -> (Vec<i32>, Vec<i32>) {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    for line in input.lines() {
        let mut numbers = line.split_whitespace();
        left_list.push(numbers.next().unwrap().parse().unwrap());
        right_list.push(numbers.next().unwrap().parse().unwrap());
    }
    (left_list, right_list)
}

fn part_one() -> i32 {
    let (left_list, right_list) = read_input_two_lists();
    let mut left_list_sorted = left_list.clone();
    left_list_sorted.sort();
    let mut right_list_sorted = right_list.clone();
    right_list_sorted.sort();
    let mut sum_dist = 0;
    for i in 0..left_list.len() {
        sum_dist += (left_list_sorted[i] - right_list_sorted[i]).abs();
    }
    sum_dist
}

fn part_two() -> i32 {
    let (left_list, right_list) = read_input_two_lists();
    let mut similarity_score = 0;
    for i in 0..left_list.len() {
        let mut appears_right_list = 0;
        for j in 0..left_list.len() {
            if left_list[i] == right_list[j] {
                appears_right_list += 1;
            }
        }
        similarity_score += left_list[i] * appears_right_list;
    }
    similarity_score
}

fn main() {
    println!("----- AOC 2024 DAY 1 -----");
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}
