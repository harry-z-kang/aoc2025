use std::fs;

fn main() {
    let input = fs::read_to_string("day2/input.txt").unwrap();
    let ranges = input
        .trim()
        .split(",")
        .map(|range_str| {
            range_str
                .split("-")
                .map(|range_num| range_num.parse::<i64>().expect("Invalid Input"))
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    let mut sum_of_invalid_num = 0;

    for range in ranges.clone() {
        for num in range[0]..(range[1] + 1) {
            let num_str = num.to_string();

            if num_str.len() % 2 == 1 {
                continue;
            }

            let num_part1 = &num_str[..num_str.len() / 2];
            let num_part2 = &num_str[num_str.len() / 2..];

            if num_part1 == num_part2 {
                sum_of_invalid_num += num;
            }
        }
    }

    println!("Part 1: {}", sum_of_invalid_num);

    let mut sum_of_invalid_num = 0;

    for range in ranges.clone() {
        for num in range[0]..(range[1] + 1) {
            let num_str = num.to_string();
            let mut is_invalid = false;

            for sub_str_i in 0..num_str.len() {
                let sub_str = &num_str[..sub_str_i];

                if (num_str.split(sub_str).count() - 1) * sub_str.len() == num_str.len() {
                    sum_of_invalid_num += num;
                    is_invalid = true;
                    break;
                }
            }

            if is_invalid {
                continue;
            }
        }
    }

    println!("Part 2: {}", sum_of_invalid_num);
}
