use std::{
    collections::{BTreeMap, HashMap},
    fs,
};

use itertools::Itertools;

type Coord3 = (u64, u64, u64);
type Pair = (Coord3, Coord3);

fn main() {
    let input = fs::read_to_string("day8/input.txt").unwrap();

    let mut circuit_id = 0;
    let mut circuits: HashMap<Coord3, usize> = HashMap::new();
    input
        .split("\n")
        .map(|line| {
            line.split(',')
                .map(|num_str| num_str.parse::<u64>().unwrap())
                .collect_tuple::<Coord3>()
                .unwrap()
        })
        .for_each(|coord| {
            circuits.insert(coord, circuit_id);
            circuit_id += 1;
        });

    let mut pairs: BTreeMap<u64, Vec<Pair>> = BTreeMap::new();
    circuits.keys().combinations(2).for_each(|pair| {
        let new_pair = (*pair[0], *pair[1]);
        pairs
            .entry(calculate_distance(new_pair))
            .or_default()
            .push(new_pair)
    });

    // Part 1
    let mut circuits_part1 = circuits.clone();

    let mut iter = 1000;
    for distance in pairs.keys() {
        if iter <= 0 {
            break;
        }

        for (coord1, coord2) in pairs.get(distance).unwrap() {
            let circuit_id1 = *circuits_part1.get(coord1).unwrap_or(&0);
            let circuit_id2 = *circuits_part1.get(coord2).unwrap_or(&0);

            if circuit_id1 == circuit_id2 {
                iter -= 1;
                continue;
            }

            // Combine Circuits
            circuits_part1
                .iter_mut()
                .filter(|(_, circuit_id)| **circuit_id == circuit_id2)
                .for_each(|(_, circuit_id)| {
                    *circuit_id = circuit_id1;
                });
            iter -= 1;
        }
    }

    let mut counts: HashMap<usize, u64> = HashMap::new();
    for circuit_id in circuits_part1.values() {
        *counts.entry(*circuit_id).or_default() += 1;
    }

    let mut values: Vec<u64> = counts.values().copied().collect();
    values.sort();
    values.reverse();

    println!("Part 1: {}", values[0..3].iter().product::<u64>());

    // Part 2
    let mut circuits_part2 = circuits.clone();

    let mut last_junction: Option<(u64, u64)> = None;

    pairs.keys().any(|distance| {
        for (coord1, coord2) in pairs.get(distance).unwrap() {
            let circuit_id1 = *circuits_part2.get(coord1).unwrap_or(&0);
            let circuit_id2 = *circuits_part2.get(coord2).unwrap_or(&0);

            if circuit_id1 == circuit_id2 {
                continue;
            }

            // Combine Circuits
            circuits_part2
                .iter_mut()
                .filter(|(_, circuit_id)| **circuit_id == circuit_id2)
                .for_each(|(_, circuit_id)| {
                    *circuit_id = circuit_id1;
                });

            if circuits_part2
                .values()
                .all(|&circuit_id| circuit_id == circuit_id1)
            {
                last_junction = Some((coord1.0, coord2.0));
                return true; // short-circuit outer loop (why I used any)
            }
        }

        false
    });

    println!(
        "Part 2: {}",
        last_junction.unwrap().0 * last_junction.unwrap().1
    );
}

fn calculate_distance(pair: Pair) -> u64 {
    (pair.0.0 - pair.1.0).pow(2) + (pair.0.1 - pair.1.1).pow(2) + (pair.0.2 - pair.1.2).pow(2)
}
