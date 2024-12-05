fn read_input_pages() -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut page_ordering_rules = Vec::new();
    let mut page_numbers_updates = Vec::new();
    let lines = input.lines().collect::<Vec<&str>>();
    let mut i = 0;
    while lines[i].trim() != "" {
        let mut page_ordering_rule = (0, 0);
        let mut split = lines[i].split("|");
        page_ordering_rule.0 = split.next().unwrap().parse().unwrap();
        page_ordering_rule.1 = split.next().unwrap().parse().unwrap();
        page_ordering_rules.push(page_ordering_rule);
        i += 1;
    }
    i += 1;
    while i < lines.len() {
        let page_numbers = lines[i].split(",").map(|n| n.parse().unwrap()).collect();
        page_numbers_updates.push(page_numbers);
        i += 1;
    }
    (page_ordering_rules, page_numbers_updates)
}

fn is_correct_ordering(
    page_ordering_rules: &Vec<(i32, i32)>,
    page_numbers_update: &Vec<i32>,
) -> bool {
    for (index, page) in page_numbers_update.iter().enumerate() {
        if index == 0 {
            // check in page_ordering_rules if there is a rule where
            // the first page is in the first postion
            let mut follow_rules = false;
            for rule in page_ordering_rules {
                if rule.0 == *page {
                    follow_rules = true;
                    break;
                }
            }
            if !follow_rules {
                return false;
            }
        } else if index == (page_numbers_update.len() - 1).try_into().unwrap() {
            // check in page_ordering_rules if there is a rule where
            // it can be after the previous one
            let mut follow_rules = false;
            for rule in page_ordering_rules {
                if rule.1 == *page {
                    if page_numbers_update[index - 1] == rule.0 {
                        follow_rules = true;
                        break;
                    }
                }
            }
            if !follow_rules {
                return false;
            }
        } else {
            // check in page_ordering_rules if can be in the middle
            // of the page_numbers_update
            let mut follow_rules = false;
            for rule in page_ordering_rules {
                if rule.1 == *page {
                    if page_numbers_update[index - 1] == rule.0 {
                        follow_rules = true;
                        break;
                    }
                }
            }
            if !follow_rules {
                return false;
            }
            // in this case we have alreeady checked if that value can be
            // after the previous one, now we have to check if can be before
            // the next one
            for rule in page_ordering_rules {
                if rule.0 == *page {
                    if page_numbers_update[index + 1] == rule.1 {
                        follow_rules = true;
                        break;
                    }
                }
            }
            if !follow_rules {
                return false;
            }
        }
    }
    true
}

fn correct_update(update: &mut Vec<i32>, page_ordering_rules: &Vec<(i32, i32)>) {
    // Use a more efficient algorithm to reorder the vector to pass the rules
    if is_correct_ordering(&page_ordering_rules, &update) {
        return;
    }

    let mut visited = std::collections::HashSet::new();
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(update.clone());

    while let Some(current) = queue.pop_front() {
        for i in 0..current.len() {
            for j in i + 1..current.len() {
                let mut next = current.clone();
                next.swap(i, j);
                if is_correct_ordering(&page_ordering_rules, &next) {
                    *update = next;
                    return;
                }
                if visited.insert(next.clone()) {
                    queue.push_back(next);
                }
            }
        }
    }

    println!("Not corrected: {:?}", update);
}

fn part_one() -> i32 {
    let (page_ordering_rules, page_numbers_updates) = read_input_pages();
    let mut sum = 0;
    for update in page_numbers_updates {
        if is_correct_ordering(&page_ordering_rules, &update) {
            sum += update[update.len() / 2];
        }
    }
    sum
}

fn part_two() -> i32 {
    let (page_ordering_rules, page_numbers_updates) = read_input_pages();
    let mut sum = 0;
    for (index, update) in page_numbers_updates.iter().enumerate() {
        println!("Updating {:?}/{:?}", index + 1, page_numbers_updates.len());
        if !is_correct_ordering(&page_ordering_rules, &update) {
            let mut update_mut = update.clone();
            correct_update(&mut update_mut, &page_ordering_rules);
            sum += update_mut[update_mut.len() / 2];
        }
    }
    sum
}

fn main() {
    println!("----- AOC 2024 DAY 5 -----");
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}
