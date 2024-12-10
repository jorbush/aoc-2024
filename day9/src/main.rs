fn read_input_disk_map() -> String {
    let input = std::fs::read_to_string("input.txt").unwrap();
    input.trim().to_string()
}

fn decompact_disk(disk_map: String) -> String {
    let mut decompacted = String::new();
    let mut current_id = 0;
    for (index, ch) in disk_map.chars().enumerate() {
        if index % 2 == 0 || index == 0 {
            //layout of files
            let file = current_id
                .to_string()
                .repeat(ch.to_digit(10).unwrap() as usize);
            decompacted.push_str(file.as_str());
            current_id += 1;
        } else {
            //layout of free space
            let free_space = ".".repeat(ch.to_digit(10).unwrap() as usize);
            decompacted.push_str(free_space.as_str());
        }
    }
    decompacted
}

fn move_file_block(decompacted_disk: &mut Vec<char>, source: usize, target: usize) {
    decompacted_disk.swap(source, target);
}

fn move_file_blocks_to_free_space(decompacted_disk: String) -> String {
    let mut moved_file: Vec<char> = decompacted_disk.chars().collect();
    let mut current_index = 0;
    while current_index < moved_file.len() {
        if moved_file[current_index] == '.' {
            for index_move_fs in (current_index + 1..moved_file.len()).rev() {
                if moved_file[index_move_fs] != '.' {
                    move_file_block(&mut moved_file, index_move_fs, current_index);
                    // println!("{}", moved_file.iter().collect::<String>());
                    break;
                }
            }
        }
        current_index += 1;
    }
    moved_file.iter().collect()
}

fn update_filesystem_checksum(moved_files_disk: String) -> i64 {
    let mut checksum: i64 = 0;
    for (index, block) in moved_files_disk.chars().enumerate() {
        if block == '.' {
            break;
        }
        checksum += index as i64 * block.to_digit(10).unwrap() as i64;
    }
    checksum
}

fn part_one() -> i64 {
    let disk_map = read_input_disk_map();
    let decompacted_disk = decompact_disk(disk_map);
    let moved_files_disk = move_file_blocks_to_free_space(decompacted_disk);
    update_filesystem_checksum(moved_files_disk)
}

fn part_two() -> i64 {
    0
}

fn main() {
    println!("----- AOC 2024 DAY 9 -----");
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}
