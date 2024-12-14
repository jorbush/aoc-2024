const COST_BUTTON_A: i8 = 3;
const COST_BUTTON_B: i8 = 1;
const MAX_PRESSES: i8 = 100;

enum ReadLineType {
    Button,
    Prize,
}

fn read_machine_line(line: &str, read_type: ReadLineType) -> (i16, i16) {
    // example: Button A: X+94, Y+34
    // example: Prize: X=8400, Y=5400
    let mut position = (0, 0);
    let values: Vec<&str> = match read_type {
        ReadLineType::Button => line.split("+").collect(),
        ReadLineType::Prize => line.split("=").collect(),
    };
    position.0 = values[1].split(",").next().unwrap().parse().unwrap();
    position.1 = values[2].parse().unwrap();
    position
}

fn read_input_machines() -> Vec<((i16, i16), (i16, i16), (i16, i16))> {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut machines = Vec::new();
    let lines = input.lines().collect::<Vec<&str>>();
    let mut i = 0;
    while i < lines.len() {
        let button_a = read_machine_line(lines[i], ReadLineType::Button);
        i += 1;
        let button_b = read_machine_line(lines[i], ReadLineType::Button);
        i += 1;
        let prize = read_machine_line(lines[i], ReadLineType::Prize);
        i += 1;
        machines.push((button_a, button_b, prize));
        i += 1;
    }
    machines
}

fn calculate_min_tokens(
    current_position: (i16, i16),
    target_position: (i16, i16),
    a_values: (i16, i16),
    b_values: (i16, i16),
    current_tokens: i32,
    current_presses: i8,
) -> i32 {
    println!(
        "Current position: {:?}, target position: {:?}, current tokens: {}, current presses: {}",
        current_position, target_position, current_tokens, current_presses
    );
    if current_position.0 > target_position.0
        || current_position.1 > target_position.1
        || current_presses > MAX_PRESSES
    {
        return 0;
    }
    if current_position == target_position {
        return current_tokens;
    }
    let tokens_a = calculate_min_tokens(
        (
            current_position.0 + a_values.0,
            current_position.1 + a_values.1,
        ),
        target_position,
        a_values,
        b_values,
        current_tokens + COST_BUTTON_A as i32,
        current_presses + 1,
    );
    let tokens_b = calculate_min_tokens(
        (
            current_position.0 + b_values.0,
            current_position.1 + b_values.1,
        ),
        target_position,
        a_values,
        b_values,
        current_tokens + COST_BUTTON_B as i32,
        current_presses + 1,
    );
    match (tokens_a, tokens_b) {
        (0, 0) => 0,
        (0, b) => b,
        (a, 0) => a,
        (a, b) => a.min(b),
    }
}

fn calculate_tokens(machine: ((i16, i16), (i16, i16), (i16, i16))) -> i32 {
    let button_a_values = machine.0;
    let button_b_values = machine.1;
    let target_position = machine.2;
    calculate_min_tokens(
        (0, 0),
        target_position,
        button_a_values,
        button_b_values,
        0,
        0,
    )
}

fn part_one() -> i32 {
    let machines = read_input_machines();
    println!("{:?}", machines);
    let mut fewest_tokens = 0;
    for machine in machines.iter() {
        fewest_tokens += calculate_tokens(*machine);
    }
    fewest_tokens
}

fn part_two() -> i32 {
    0
}

fn main() {
    println!("----- AOC 2024 DAY 13 -----");
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}
