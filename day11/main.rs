use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("day11/input.txt").unwrap();

    let mut route_mapping = HashMap::<&str, Vec<&str>>::new();
    input.split("\n").for_each(|line| {
        let parts = line.split(": ").collect::<Vec<&str>>();
        route_mapping.insert(parts[0], parts[1].split(" ").collect());
    });

    println!(
        "Part 1: {}",
        advance_part1(
            "you",
            &mut Vec::new(),
            &route_mapping,
            &mut HashMap::<String, u64>::new()
        )
    );

    println!(
        "Part 2: {}",
        advance_part2(
            "svr",
            false,
            false,
            &mut Vec::new(),
            &route_mapping,
            &mut HashMap::<String, u64>::new(),
            0
        )
    );
}

fn advance_part1(
    curr_dev: &str,
    visited: &mut Vec<&str>,
    route_mapping: &HashMap<&str, Vec<&str>>,
    cache: &mut HashMap<String, u64>,
) -> u64 {
    if curr_dev == "out" {
        return 1;
    }

    if visited.contains(&curr_dev) {
        return 0;
    }

    let key = hash_cache_key(curr_dev, false, false, visited);

    if cache.contains_key(&key) {
        return cache[&key];
    }

    let num_routes = route_mapping[curr_dev]
        .iter()
        .map(|next_dev| advance_part1(next_dev, &mut visited.clone(), route_mapping, cache))
        .sum();

    cache.insert(key, num_routes);

    num_routes
}

fn advance_part2(
    curr_dev: &str,
    visited_fft: bool,
    visited_dac: bool,
    visited: &mut Vec<&str>,
    route_mapping: &HashMap<&str, Vec<&str>>,
    cache: &mut HashMap<String, u64>,
    level: u64,
) -> u64 {
    if curr_dev == "out" {
        return (visited_fft && visited_dac) as u64;
    }

    if visited.contains(&curr_dev) {
        return 0;
    }

    let key = hash_cache_key(curr_dev, visited_fft, visited_dac, visited);

    if cache.contains_key(&key) {
        return cache[&key];
    }

    let num_routes = route_mapping[curr_dev]
        .iter()
        .map(|next_dev| {
            advance_part2(
                next_dev,
                curr_dev == "fft" || visited_fft,
                curr_dev == "dac" || visited_dac,
                &mut visited.clone(),
                route_mapping,
                cache,
                level + 1,
            )
        })
        .sum();

    cache.insert(key, num_routes);

    num_routes
}

fn hash_cache_key(
    curr_dev: &str,
    visited_fft: bool,
    visited_dac: bool,
    visited: &mut Vec<&str>,
) -> String {
    visited.sort();

    let key = curr_dev.to_string();

    key + &visited.join("").to_string() + &visited_fft.to_string() + &visited_dac.to_string()
}
