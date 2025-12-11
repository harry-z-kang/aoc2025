use std::{collections::HashSet, fs};

#[derive(Debug)]
struct Machine {
    goal: u16,
    buttons: Vec<u16>,
    joltages: Vec<u16>,
}

struct Matrix {
    matrix: Vec<Vec<f64>>,
    rows: usize,
    cols: usize,
    dependents: Vec<usize>,
    independents: Vec<usize>,
}

const EPSILON: f64 = 1e-9;

fn main() {
    let input = fs::read_to_string("day10/input.txt").unwrap();

    let machines: Vec<Machine> = input
        .split("\n")
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            let indicator_str = parts[0];
            let buttons_str = &parts[1..parts.len() - 1];
            let joltage_str = parts[parts.len() - 1];

            Machine {
                goal: parse_indicator_string(indicator_str),
                buttons: parse_buttons_string(buttons_str),
                joltages: parse_joltage_string(joltage_str),
            }
        })
        .collect();

    let sum = machines
        .iter()
        .map(|machine| {
            let mut state_set = HashSet::<u16>::new();
            state_set.insert(0_u16);

            let mut push = 0;
            loop {
                state_set = state_set
                    .clone()
                    .into_iter()
                    .flat_map(|state| machine.buttons.iter().map(move |button| state ^ button))
                    .collect::<HashSet<u16>>();

                push += 1;

                if state_set.contains(&machine.goal) {
                    break;
                }
            }
            push
        })
        .sum::<u64>();

    println!("Part 1: {}", sum);

    let sum = machines
        .iter()
        .map(|machine| {
            let mut matrix = parse_machine_into_matrix(machine);

            gaussian_elimination(&mut matrix);

            // Now we can DFS over a much smaller solution space.
            let max = (*machine.joltages.iter().max().unwrap() + 1) as usize;
            let mut min = usize::MAX;
            let mut values = vec![0; matrix.independents.len()];

            dfs(&matrix, 0, &mut values, &mut min, max);

            min
        })
        .sum::<usize>();

    println!("Part 2: {}", sum);
}

#[inline]
fn parse_indicator_string(indicator_str: &str) -> u16 {
    let mut result = 0_u16;

    indicator_str
        .trim_start_matches("[")
        .trim_end_matches("]")
        .chars()
        .enumerate()
        .for_each(|(index, c)| match c {
            '.' => result |= 0_u16 << index,
            '#' => result |= 1_u16 << index,
            _ => panic!("Invalid indicator string: {indicator_str}"),
        });

    return result;
}

#[inline]
fn parse_buttons_string(buttons_str: &[&str]) -> Vec<u16> {
    buttons_str
        .into_iter()
        .map(|button_combination| {
            let mut button_comb_bin = 0_u16;
            button_combination
                .trim_start_matches("(")
                .trim_end_matches(")")
                .split(",")
                .map(|b| b.parse::<usize>().unwrap())
                .for_each(|b| button_comb_bin |= 1_u16 << b);
            return button_comb_bin;
        })
        .collect()
}

#[inline]
fn parse_joltage_string(joltage_str: &str) -> Vec<u16> {
    joltage_str
        .trim_start_matches("{")
        .trim_end_matches("}")
        .split(",")
        .map(|j| j.parse::<u16>().unwrap())
        .collect()
}

fn parse_machine_into_matrix(machine: &Machine) -> Matrix {
    let mut matrix = vec![vec![0.0; machine.buttons.len() + 1]; machine.joltages.len()];

    for (col, button) in machine.buttons.iter().enumerate() {
        for row in 0..machine.joltages.len() {
            if (button & 1 << row) != 0 {
                matrix[row][col] = 1.0;
            }
        }
    }

    for (row, value) in machine.joltages.iter().enumerate() {
        matrix[row][machine.buttons.len()] = *value as f64;
    }

    Matrix {
        matrix,
        rows: machine.joltages.len(),
        cols: machine.buttons.len(),
        dependents: Vec::new(),
        independents: Vec::new(),
    }
}

fn gaussian_elimination(matrix: &mut Matrix) {
    let mut pivot = 0;

    let mut col = 0;
    while pivot < matrix.rows && col < matrix.cols {
        // Find the best pivot row for this column.
        let (best_row, best_value) = matrix
            .matrix
            .iter()
            .enumerate()
            .skip(pivot)
            .map(|(r, row)| (r, row[col].abs()))
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();

        // If the best value is zero, this is a free variable.
        if best_value < EPSILON {
            matrix.independents.push(col);
            col += 1;
            continue;
        }

        // Swap rows and mark this column as dependent.
        matrix.matrix.swap(pivot, best_row);
        matrix.dependents.push(col);

        // Normalize pivot row.
        let pivot_value = matrix.matrix[pivot][col];
        for val in &mut matrix.matrix[pivot][col..=matrix.cols] {
            *val /= pivot_value;
        }

        // Eliminate this column in all other rows.
        for r in 0..matrix.rows {
            if r != pivot {
                let factor = matrix.matrix[r][col];
                if factor.abs() > EPSILON {
                    let pivot_row = matrix.matrix[pivot][col..=matrix.cols].to_vec();
                    matrix.matrix[r][col..=matrix.cols]
                        .iter_mut()
                        .zip(&pivot_row)
                        .for_each(|(val, &pivot_val)| {
                            *val -= factor * pivot_val;
                        });
                }
            }
        }

        pivot += 1;
        col += 1;
    }

    // Any remaining columns are free variables
    matrix.independents.extend(col..matrix.cols);
}

fn dfs(matrix: &Matrix, idx: usize, values: &mut [usize], min: &mut usize, max: usize) {
    // When we've assigned all independent variables, check if it's a valid solution.
    if idx == matrix.independents.len() {
        if let Some(total) = valid(matrix, values) {
            *min = (*min).min(total);
        }
        return;
    }

    // Try different values for the current independent variable.
    let total: usize = values[..idx].iter().sum();
    for val in 0..max {
        // Optimization: If we ever go above our min, we can't possibly do better.
        if total + val >= *min {
            break;
        }
        values[idx] = val;
        dfs(matrix, idx + 1, values, min, max);
    }
}

fn valid(matrix: &Matrix, values: &[usize]) -> Option<usize> {
    // We start with how many times we've pressed the free variables.
    let mut total = values.iter().sum::<usize>();

    // Calculate dependent variable values based on independent variables.
    for row in 0..matrix.dependents.len() {
        // Calculate this dependent by subtracting the sum of the free variable pushes from the solution.
        let val = matrix
            .independents
            .iter()
            .enumerate()
            .fold(matrix.matrix[row][matrix.cols], |acc, (i, &col)| {
                acc - matrix.matrix[row][col] * (values[i] as f64)
            });

        // We need non-negative, whole numbers for a valid solution.
        if val < -EPSILON {
            return None;
        }
        let rounded = val.round();
        if (val - rounded).abs() > EPSILON {
            return None;
        }

        total += rounded as usize;
    }

    Some(total)
}
