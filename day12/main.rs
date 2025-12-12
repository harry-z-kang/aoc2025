use std::fs;

struct Region {
    width: u64,
    length: u64,
    quantities: Vec<u64>,
}

fn main() {
    let input = fs::read_to_string("day12/input.txt").unwrap();
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let shapes_str = &parts[0..6];
    let regions_str = parts[6];

    let shapes: Vec<Vec<Vec<char>>> = shapes_str
        .into_iter()
        .map(|present_str| {
            present_str
                .split("\n")
                .skip(1)
                .map(|line| line.chars().collect())
                .collect()
        })
        .collect();
    let regions: Vec<Region> = regions_str
        .split("\n")
        .map(|line| {
            let parts = line.split(": ").collect::<Vec<&str>>();
            let size_parts = parts[0].split("x").collect::<Vec<&str>>();
            let width = size_parts[0].parse::<u64>().unwrap();
            let length = size_parts[1].parse::<u64>().unwrap();

            let quantities = parts[1]
                .split(" ")
                .map(|num_str| num_str.parse::<u64>().unwrap())
                .collect();

            Region {
                width,
                length,
                quantities,
            }
        })
        .collect();

    println!(
        "Part 1: {}",
        regions
            .iter()
            .filter(|region| {
                region
                    .quantities
                    .iter()
                    .enumerate()
                    .map(|(index, quantity)| quantity * get_occupied_tiles(&shapes[index]))
                    .sum::<u64>()
                    < region.width * region.length
            })
            .count()
    );
}

fn get_occupied_tiles(present: &Vec<Vec<char>>) -> u64 {
    present
        .iter()
        .map(|row| row.iter().filter(|c| **c == '#').count() as u64)
        .sum::<u64>()
}
