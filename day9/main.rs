use itertools::Itertools;
use std::fs;

type Coord = (u64, u64);

fn main() {
    let input = fs::read_to_string("day9/input.txt").unwrap();
    let coordinates: Vec<Coord> = input
        .split("\n")
        .map(|line| {
            line.split(",")
                .map(|num_str| num_str.parse::<u64>().unwrap())
                .collect_tuple::<Coord>()
                .unwrap()
        })
        .collect();

    let mut max_area = 0;
    coordinates.iter().combinations(2).for_each(|pair| {
        let area = calculate_rectangle_area(*pair[0], *pair[1]);
        if area > max_area {
            max_area = area
        }
    });

    println!("Part 1: {}", max_area);

    let lines = coordinates
        .iter()
        .circular_tuple_windows()
        .collect::<Vec<(&Coord, &Coord)>>();
    let max_area = coordinates
        .iter()
        .tuple_combinations()
        .map(|(coord1, coord2)| (coord1, coord2, calculate_rectangle_area(*coord1, *coord2)))
        .sorted_by_key(|v| v.2)
        .rev()
        .find(|(coord1, coord2, _)| {
            lines.iter().all(|(line_start, line_end)| {
                // Check if the rectangle is boxed in
                let left_of_rect = coord1.1.max(coord2.1) <= line_start.1.min(line_end.1);
                let right_of_rect = coord1.1.min(coord2.1) >= line_start.1.max(line_end.1);
                let above = coord1.0.max(coord2.0) <= line_start.0.min(line_end.0);
                let below = coord1.0.min(coord2.0) >= line_start.0.max(line_end.0);
                left_of_rect || right_of_rect || above || below
            })
        })
        .unwrap()
        .2;

    println!("Part 2: {}", max_area);
}

fn calculate_rectangle_area(coord1: Coord, coord2: Coord) -> u64 {
    (coord1.0.abs_diff(coord2.0) + 1) * (coord1.1.abs_diff(coord2.1) + 1)
}
