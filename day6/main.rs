use std::fs;

fn main() {
    let input = fs::read_to_string("day6/input.txt").unwrap();
    let mut lines = input
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut pre_transpose_operands = Vec::<Vec<i64>>::new();
    for line_num in 0..lines.len() - 1 {
        pre_transpose_operands.push(
            lines[line_num]
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>(),
        );
    }
    let operands: Vec<Vec<i64>> = (0..pre_transpose_operands[0].len())
        .map(|i| {
            pre_transpose_operands
                .iter()
                .map(|row| row[i])
                .collect::<Vec<i64>>()
        })
        .collect();
    let operator = lines[lines.len() - 1]
        .split_whitespace()
        .collect::<Vec<&str>>();

    let mut total = 0;

    for (index, op) in operator.iter().enumerate() {
        total += match *op {
            "+" => operands[index].iter().sum::<i64>(),
            "*" => operands[index].iter().product::<i64>(),
            _ => panic!("Unsupported operator"),
        };
    }

    println!("Part 1: {}", total);

    for i in 0..lines.iter().map(|s| s.len()).max().unwrap() {
        let column_should_zero_filled = lines
            .iter()
            .map(|s| s.chars().nth(i).unwrap_or(' '))
            .any(|c| c != ' ');

        if column_should_zero_filled {
            for line_num in 0..lines.len() - 1 {
                let mut chars: Vec<char> = lines[line_num].chars().collect();
                if chars[i] == ' ' {
                    chars[i] = '0';
                }
                lines[line_num] = chars.into_iter().collect();
            }
        }
    }

    let mut pre_transpose_operands = Vec::<Vec<String>>::new();
    for line_num in 0..lines.len() - 1 {
        pre_transpose_operands.push(
            lines[line_num]
                .split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        );
    }
    let operands: Vec<Vec<i64>> = (0..pre_transpose_operands[0].len())
        .map(|i| {
            pre_transpose_operands
                .iter()
                .map(|row| row[i].clone())
                .collect::<Vec<String>>()
        })
        .map(|horizontal_operands| {
            (0..horizontal_operands[0].len())
                .map(|j| {
                    horizontal_operands
                        .iter()
                        .map(|s| s.chars().nth(j).unwrap())
                        .collect::<String>()
                })
                .map(|str_num| {
                    str_num
                        .trim_start_matches('0')
                        .trim_end_matches('0')
                        .parse::<i64>()
                        .unwrap()
                })
                .collect::<Vec<i64>>()
        })
        .collect();
    let operator = lines[lines.len() - 1]
        .split_whitespace()
        .collect::<Vec<&str>>();

    let mut total = 0;

    for (index, op) in operator.iter().enumerate() {
        total += match *op {
            "+" => operands[index].iter().sum::<i64>(),
            "*" => operands[index].iter().product::<i64>(),
            _ => panic!("Unsupported operator"),
        };
    }

    println!("Part 2: {}", total);
}
