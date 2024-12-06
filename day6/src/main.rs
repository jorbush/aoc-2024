use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn read_input_map() -> Vec<Vec<String>> {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut map = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c.to_string());
        }
        map.push(row);
    }
    map
}

fn get_guard_position_and_direction(map: &Vec<Vec<String>>) -> ((i32, i32), Direction) {
    let mut guard_position = (0, 0);
    let mut direction = Direction::Up;
    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            match c.as_str() {
                "^" => {
                    guard_position = (i as i32, j as i32);
                    direction = Direction::Up;
                }
                "v" => {
                    guard_position = (i as i32, j as i32);
                    direction = Direction::Down;
                }
                "<" => {
                    guard_position = (i as i32, j as i32);
                    direction = Direction::Left;
                }
                ">" => {
                    guard_position = (i as i32, j as i32);
                    direction = Direction::Right;
                }
                _ => (),
            }
        }
    }
    (guard_position, direction)
}

fn move_guard_turn_right_90_degrees(
    map: &mut Vec<Vec<String>>,
    guard_position: (i32, i32),
    direction: &mut Direction,
) {
    match direction {
        Direction::Up => {
            *direction = Direction::Right;
            map[guard_position.0 as usize][guard_position.1 as usize] = String::from(">");
        }
        Direction::Down => {
            *direction = Direction::Left;
            map[guard_position.0 as usize][guard_position.1 as usize] = String::from("<");
        }
        Direction::Left => {
            *direction = Direction::Up;
            map[guard_position.0 as usize][guard_position.1 as usize] = String::from("^");
        }
        Direction::Right => {
            *direction = Direction::Down;
            map[guard_position.0 as usize][guard_position.1 as usize] = String::from("v");
        }
    }
}

fn can_move_forward(
    map: &Vec<Vec<String>>,
    guard_position: (i32, i32),
    direction: Direction,
) -> bool {
    match direction {
        Direction::Up => {
            match map[guard_position.0 as usize - 1][guard_position.1 as usize].as_str() {
                "#" => false,
                _ => true,
            }
        }
        Direction::Down => {
            match map[guard_position.0 as usize + 1][guard_position.1 as usize].as_str() {
                "#" => false,
                _ => true,
            }
        }
        Direction::Left => {
            match map[guard_position.0 as usize][guard_position.1 as usize - 1].as_str() {
                "#" => false,
                _ => true,
            }
        }
        Direction::Right => {
            match map[guard_position.0 as usize][guard_position.1 as usize + 1].as_str() {
                "#" => false,
                _ => true,
            }
        }
    }
}

fn is_going_to_leave_map(
    map: &Vec<Vec<String>>,
    guard_position: (i32, i32),
    direction: Direction,
) -> bool {
    match direction {
        Direction::Up => guard_position.0 == 0,
        Direction::Down => guard_position.0 == map.len() as i32 - 1,
        Direction::Left => guard_position.1 == 0,
        Direction::Right => guard_position.1 == map[0].len() as i32 - 1,
    }
}

fn move_step_forward(
    map: &mut Vec<Vec<String>>,
    guard_position: &mut (i32, i32),
    direction: &mut Direction,
    guard_route: &mut HashMap<(i32, i32), i32>,
) {
    let new_position = match direction {
        Direction::Up => {
            map[guard_position.0 as usize][guard_position.1 as usize] = String::from(".");
            (guard_position.0 - 1, guard_position.1)
        }
        Direction::Down => {
            map[guard_position.0 as usize][guard_position.1 as usize] = String::from(".");
            (guard_position.0 + 1, guard_position.1)
        }
        Direction::Left => {
            map[guard_position.0 as usize][guard_position.1 as usize] = String::from(".");
            (guard_position.0, guard_position.1 - 1)
        }
        Direction::Right => {
            map[guard_position.0 as usize][guard_position.1 as usize] = String::from(".");
            (guard_position.0, guard_position.1 + 1)
        }
    };
    guard_route.insert(new_position, 1);
    *guard_position = new_position;
}

fn move_guard(
    map: &mut Vec<Vec<String>>,
    guard_position: &mut (i32, i32),
    guard_route: &mut HashMap<(i32, i32), i32>,
    direction: &mut Direction,
) {
    if can_move_forward(map, *guard_position, *direction) {
        move_step_forward(map, guard_position, direction, guard_route);
    } else {
        move_guard_turn_right_90_degrees(map, *guard_position, direction);
    }
}

fn get_guard_route(map: &mut Vec<Vec<String>>) -> HashMap<(i32, i32), i32> {
    let (guard_position, direction) = get_guard_position_and_direction(map);
    let mut guard_route = HashMap::new();
    guard_route.insert(guard_position, 1);
    let mut guard_position = guard_position;
    let mut direction = direction;
    loop {
        move_guard(map, &mut guard_position, &mut guard_route, &mut direction);
        if is_going_to_leave_map(map, guard_position, direction) {
            break;
        }
    }
    guard_route
}

fn part_one() -> i32 {
    let mut map = read_input_map();
    let guard_route = get_guard_route(&mut map);
    guard_route.len() as i32
}

fn part_two() -> i32 {
    0
}

fn main() {
    println!("----- AOC 2024 DAY 6 -----");
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}
