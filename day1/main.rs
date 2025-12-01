use std::fs;

const STARTING_POSITION: i32 = 50;
const MAX_POSITION: i32 = 100;

fn main() {
    let input = fs::read_to_string("day1/input.txt").unwrap();
    let instructions = parse_instructions(&input);

    let part1 = count_zero_hits(&instructions, StepMode::Chunked);
    println!("Part 1: {}", part1);

    let part2 = count_zero_hits(&instructions, StepMode::UnitSteps);
    println!("Part 2: {}", part2);
}

#[derive(Clone, Copy)]
enum StepMode {
    Chunked,
    UnitSteps,
}

#[derive(Clone, Copy)]
enum Direction {
    Left = -1,
    Right = 1,
}

fn parse_instructions(input: &str) -> Vec<(Direction, i32)> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut chars = line.chars();
            let direction = chars.next().expect("missing direction");
            let steps = line[1..].trim().parse::<i32>().expect("invalid step count");
            let delta = match direction {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("Invalid direction: {}", direction),
            };
            (delta, steps)
        })
        .collect()
}

fn count_zero_hits(instructions: &[(Direction, i32)], mode: StepMode) -> i32 {
    let mut position = STARTING_POSITION;
    let mut number_of_zeros = 0;

    for &(direction, steps) in instructions {
        match mode {
            StepMode::Chunked => {
                position = advance(position, (direction as i32) * steps);
                if position == 0 {
                    number_of_zeros += 1;
                }
            }
            StepMode::UnitSteps => {
                for _ in 0..steps {
                    position = advance(position, direction as i32);
                    if position == 0 {
                        number_of_zeros += 1;
                    }
                }
            }
        }
    }

    number_of_zeros
}

fn advance(position: i32, delta: i32) -> i32 {
    (position + delta).rem_euclid(MAX_POSITION)
}
