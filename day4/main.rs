use std::fs;

fn main() {
    let input = fs::read_to_string("day4/input.txt").unwrap();
    let mut map = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut accessible_rolls = 0;
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            accessible_rolls += check_accessbility(row, col, &map) as i32;
        }
    }

    println!("Part 1: {}", accessible_rolls);

    let mut accessible_rolls = 0;

    loop {
        let mut current_cycle_accessible_rolls = 0;
        let mut coords_to_be_removed = Vec::<(usize, usize)>::new();

        for row in 0..map.len() {
            for col in 0..map[0].len() {
                if check_accessbility(row, col, &map) {
                    current_cycle_accessible_rolls += 1;
                    coords_to_be_removed.push((row, col));
                }
            }
        }

        for (row, col) in coords_to_be_removed {
            map[row][col] = '.';
        }

        accessible_rolls += current_cycle_accessible_rolls;

        if current_cycle_accessible_rolls == 0 {
            break;
        }
    }

    println!("Part 2: {}", accessible_rolls);
}

fn check_accessbility(row: usize, col: usize, map: &Vec<Vec<char>>) -> bool {
    if map[row][col] != '@' {
        return false;
    }

    let mut adjacent_rolls = 0;

    if row > 0 {
        adjacent_rolls += (map[row - 1][col] == '@') as i32;

        if col > 0 {
            adjacent_rolls += (map[row - 1][col - 1] == '@') as i32;
        }

        if col < map[0].len() - 1 {
            adjacent_rolls += (map[row - 1][col + 1] == '@') as i32;
        }
    }

    if row < map.len() - 1 {
        adjacent_rolls += (map[row + 1][col] == '@') as i32;

        if col > 0 {
            adjacent_rolls += (map[row + 1][col - 1] == '@') as i32;
        }

        if col < map[0].len() - 1 {
            adjacent_rolls += (map[row + 1][col + 1] == '@') as i32;
        }
    }

    if col > 0 {
        adjacent_rolls += (map[row][col - 1] == '@') as i32;
    }

    if col < map[0].len() - 1 {
        adjacent_rolls += (map[row][col + 1] == '@') as i32;
    }

    return adjacent_rolls < 4;
}
