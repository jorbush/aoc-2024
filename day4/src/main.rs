fn read_input_words() -> Vec<String> {
    let input = std::fs::read_to_string("input.txt").unwrap();
    input.lines().map(|line| line.to_string()).collect()
}

fn transpose(grid: &Vec<String>) -> Vec<String> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut transposed: Vec<String> = Vec::new();

    for col in 0..cols {
        let mut new_row = String::new();
        for row in 0..rows {
            new_row.push(grid[row].chars().nth(col).unwrap());
        }
        transposed.push(new_row);
    }
    transposed
}

fn diagonals(grid: &Vec<String>) -> Vec<String> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut result: Vec<String> = Vec::new();
    for start_col in 0..cols {
        let mut diagonal = String::new();
        let mut row = 0;
        let mut col = start_col;
        while col < cols && row < rows {
            diagonal.push(grid[row].chars().nth(col).unwrap());
            row += 1;
            col += 1;
        }
        result.push(diagonal);
    }
    for start_row in 1..rows {
        let mut diagonal = String::new();
        let mut row = start_row;
        let mut col = 0;
        while col < cols && row < rows {
            diagonal.push(grid[row].chars().nth(col).unwrap());
            row += 1;
            col += 1;
        }
        result.push(diagonal);
    }
    result
}

fn reverse_diagonals(grid: &Vec<String>) -> Vec<String> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut result: Vec<String> = Vec::new();

    // Diagonales desde la parte superior derecha hacia abajo izquierda
    for start_col in (0..cols).rev() {
        let mut diagonal = String::new();
        let mut row = 0;
        let mut col = start_col;
        while col < cols && row < rows {
            diagonal.push(grid[row].chars().nth(col).unwrap());
            row += 1;
            col = col.saturating_sub(1);
        }
        result.push(diagonal);
    }

    // Diagonales desde la parte superior izquierda hacia abajo izquierda
    for start_row in 1..rows {
        let mut diagonal = String::new();
        let mut row = start_row;
        let mut col = cols - 1;
        while row < rows && col < cols {
            diagonal.push(grid[row].chars().nth(col).unwrap());
            row += 1;
            col = col.saturating_sub(1);
        }
        result.push(diagonal);
    }

    result
}

fn reverse_words(words: &Vec<String>) -> Vec<String> {
    words
        .iter()
        .map(|line| line.chars().rev().collect())
        .collect()
}

fn search_for_word(word: &str, words: &Vec<String>) -> i32 {
    let mut count = 0;
    // Horizontal and reverse horizontal
    for line in words {
        if line.contains(word) {
            count += 1;
        }
    }
    for line in reverse_words(&words.clone()) {
        if line.contains(word) {
            count += 1;
        }
    }
    // Vertical and reverse vertical
    let transposed = transpose(&words.clone());
    for line in &transposed {
        if line.contains(word) {
            count += 1;
        }
    }
    for line in reverse_words(&transposed) {
        if line.contains(word) {
            count += 1;
        }
    }
    // Diagonals (\) and reverse diagonal (\)
    let diagonals_lr = diagonals(&words.clone());
    for line in &diagonals_lr {
        if line.contains(word) {
            count += 1;
        }
    }
    for line in reverse_words(&diagonals_lr) {
        if line.contains(word) {
            count += 1;
        }
    }
    // Diagonals (/) and reverse diagonal (/)
    let diagonals_rl = reverse_diagonals(&words.clone());
    for line in &diagonals_rl {
        if line.contains(word) {
            count += 1;
        }
    }
    for line in reverse_words(&diagonals_rl) {
        if line.contains(word) {
            count += 1;
        }
    }
    count
}

fn part_one() -> i32 {
    let words = read_input_words();
    search_for_word("XMAS", &words)
}

fn part_two() -> i32 {
    0
}

fn main() {
    println!("----- AOC 2024 DAY 4 -----");
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}
