use regex::Regex;

fn read_input_corrupted() -> String {
    let input = std::fs::read_to_string("input.txt").unwrap();
    input.trim().to_string()
}

fn get_mults(input: String) -> Vec<(i32, i32)> {
    // input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
    // output = [(2, 4), (5, 5), (11, 8), (8, 5)]
    let mut mults = Vec::new();
    // use slice windows to find the indexes of 'mul(z, y)'
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    for cap in re.captures_iter(&input) {
        let x: i32 = cap[1].parse().unwrap();
        let y: i32 = cap[2].parse().unwrap();
        mults.push((x, y));
    }
    mults
}

fn sum_mults(mults: Vec<(i32, i32)>) -> i32 {
    let mut sum = 0;
    for (x, y) in mults {
        sum += x * y;
    }
    sum
}

fn part_one() -> i32 {
    let input = read_input_corrupted();
    let mults = get_mults(input);
    sum_mults(mults)
}

fn part_two() -> i32 {
    0
}

fn main() {
    println!("----- AOC 2024 DAY 3 -----");
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}
