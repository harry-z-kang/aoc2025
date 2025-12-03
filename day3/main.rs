use std::fs;

fn main() {
    let input = fs::read_to_string("day3/input.txt").unwrap();
    let banks = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.chars()
                .map(|num| num.to_digit(10).expect("Invalid Input") as i64)
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    println!("Part 1: {}", calculate_total_joltage(2, &banks));

    println!("Part 2: {}", calculate_total_joltage(12, &banks));
}

fn calculate_total_joltage(num_of_digit: usize, banks: &Vec<Vec<i64>>) -> i64 {
    let mut output_joltage = 0;

    for joltages in banks {
        let mut sliced_joltages = joltages.clone();

        for n in (0..num_of_digit).rev() {
            let (digit, slice_index) = find_nax_nth_digit(n, &sliced_joltages);

            output_joltage += digit * 10_i64.pow(n as u32);
            sliced_joltages = sliced_joltages[slice_index + 1..].to_vec();
        }
    }

    return output_joltage;
}

fn find_nax_nth_digit(n: usize, joltages: &Vec<i64>) -> (i64, usize) {
    let max_joltage = *joltages[..joltages.len() - n]
        .iter()
        .max()
        .expect("Invalid Input");
    let max_joltage_first_index = joltages
        .iter()
        .position(|n| *n == max_joltage)
        .expect("Invalid Input");

    return (max_joltage, max_joltage_first_index);
}
