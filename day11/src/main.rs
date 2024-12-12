fn read_input_initial_stones() -> Vec<i64> {
    let input = std::fs::read_to_string("input.txt").unwrap();
    input
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

fn tranform_stones(stones: Vec<i64>) -> Vec<i64> {
    let mut new_arrangement = stones.clone();
    let mut split_stone = false;
    let mut i = 0;
    while i < new_arrangement.len() {
        if split_stone {
            split_stone = false;
        } else if new_arrangement[i] == 0 {
            new_arrangement[i] = 1;
        } else if new_arrangement[i].to_string().len() % 2 == 0 {
            let mut new_stone = new_arrangement[i].to_string();
            let half = new_stone.len() / 2;
            let left = new_stone.split_off(half);
            new_arrangement[i] = left.parse::<i64>().unwrap();
            new_arrangement.insert(i, new_stone.parse::<i64>().unwrap());
            split_stone = true;
        } else {
            let new_stone = new_arrangement[i] * 2024;
            new_arrangement[i] = new_stone;
        }
        i += 1;
    }
    new_arrangement
}

fn blink_n_times(stones: Vec<i64>, n: i64) -> Vec<i64> {
    let mut new_stones = stones.clone();
    for _ in 0..n {
        new_stones = tranform_stones(new_stones);
    }
    new_stones
}

fn part_one() -> i64 {
    let initial_stones = read_input_initial_stones();
    let final_stones = blink_n_times(initial_stones, 25);
    final_stones.len() as i64
}

fn part_two() -> i64 {
    let initial_stones = read_input_initial_stones();
    let final_stones = blink_n_times(initial_stones, 75);
    final_stones.len() as i64
}

fn main() {
    println!("----- AOC 2024 DAY 11 -----");
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}
