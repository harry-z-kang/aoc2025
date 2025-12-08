use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let input = fs::read_to_string("day7/input.txt").unwrap();
    let map = input
        .split("\n")
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut starting_position: Option<(usize, usize)> = None;
    for (row_index, row) in map.iter().enumerate() {
        for (col_index, &ch) in row.iter().enumerate() {
            if ch == 'S' {
                starting_position = Some((row_index, col_index));
            }
        }
    }

    let mut beam_positions = HashSet::<(usize, usize)>::new();
    beam_positions.insert(starting_position.expect("Can't find starting position"));

    let mut split_times = 0;

    for _ in 1..map.len() / 2 {
        let positions: Vec<(usize, usize)> = beam_positions.iter().copied().collect();
        for (row, col) in positions {
            beam_positions.remove(&(row, col));

            match map[row + 2][col] {
                '.' => {
                    beam_positions.insert((row + 2, col));
                }
                '^' => {
                    beam_positions.insert((row + 2, col - 1));
                    beam_positions.insert((row + 2, col + 1));

                    split_times += 1;
                }
                _ => panic!("Invalid Character Map"),
            }
        }
    }

    println!("Part 1: {}", split_times);

    let mut timeline_cache = HashMap::<(usize, usize), i64>::new();

    let num_timeline = step(
        starting_position.expect("Can't find starting position"),
        &map,
        &mut timeline_cache,
    );

    println!("Part 2: {}", num_timeline)
}

fn step(starting_position: (usize, usize), map: &Vec<Vec<char>>, timeline_cache: &mut HashMap::<(usize, usize), i64>) -> i64 {
    let (row, col) = starting_position;

    if row + 1 == map.len() {
        return 1;
    }

    let mut result = 0;

    match map[row + 1][col] {
        '.' => {
            if timeline_cache.contains_key(&(row + 1, col)) {
                result += timeline_cache[&(row + 1, col)];
            } else {
                let num_timelines = step((row + 1, col), map, timeline_cache);
                timeline_cache.insert((row + 1, col), num_timelines);
                result += timeline_cache[&(row + 1, col)];
            }
        }
        '^' => {
            if timeline_cache.contains_key(&(row + 1, col - 1)) {
                result += timeline_cache[&(row + 1, col - 1)];
            } else {
                let num_timelines = step((row + 1, col - 1), map, timeline_cache);
                timeline_cache.insert((row + 1, col - 1), num_timelines);
                result += timeline_cache[&(row + 1, col - 1)];
            }
            if timeline_cache.contains_key(&(row + 1, col + 1)) {
                result += timeline_cache[&(row + 1, col + 1)];
            } else {
                let num_timelines = step((row + 1, col + 1), map, timeline_cache);
                timeline_cache.insert((row + 1, col + 1), num_timelines);
                result += timeline_cache[&(row + 1, col + 1)];
            }
        }
        _ => panic!("Invalid Character Map"),
    }

    return result;
}
