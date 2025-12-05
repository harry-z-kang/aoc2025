use std::fs;

fn main() {
    let input = fs::read_to_string("day5/input.txt").unwrap();
    let splitted_input: Vec<&str> = input.split("\n\n").collect();
    let range_str = splitted_input[0];
    let id_str = splitted_input[1];

    let ranges: Vec<Vec<i64>> = range_str
        .split("\n")
        .map(|range_line| {
            range_line
                .split("-")
                .map(|range_num| range_num.parse::<i64>().expect("Invalid Input"))
                .collect::<Vec<i64>>()
        })
        .collect();
    let ids: Vec<i64> = id_str
        .split("\n")
        .map(|id_num| id_num.parse::<i64>().expect("Invalid Input"))
        .collect();

    let mut available_ingredients = 0;

    for id in ids {
        for range in &ranges {
            if id >= range[0] && id <= range[1] {
                available_ingredients += 1;
                break;
            }
        }
    }

    println!("Part 1: {}", available_ingredients);

    let mut sorted_ranges = ranges.clone();
    sorted_ranges.sort_by(|range1, range2| range1[0].cmp(&range2[0]));

    let mut i = 0;
    while i < sorted_ranges.len() - 1 {
        let range1 = &sorted_ranges[i];
        let range2 = &sorted_ranges[i + 1];

        if can_merge(range1, range2) {
            sorted_ranges[i] = merge_ranges(range1, range2);
            sorted_ranges.remove(i + 1);
        } else {
            i += 1;
        }
    }

    println!(
        "Part 2: {}",
        sorted_ranges
            .iter()
            .map(|range| range[1] - range[0] + 1)
            .sum::<i64>()
    );
}

fn can_merge(range1: &Vec<i64>, range2: &Vec<i64>) -> bool {
    assert_eq!(range1.len(), 2);
    assert_eq!(range2.len(), 2);

    range1[0] >= range2[0] && range1[0] <= range2[1]
        || range1[1] >= range2[0] && range1[1] <= range2[1]
        || range2[0] >= range1[0] && range2[0] <= range1[1]
        || range2[1] >= range1[0] && range2[1] <= range1[1]
}

fn merge_ranges(range1: &Vec<i64>, range2: &Vec<i64>) -> Vec<i64> {
    assert_eq!(range1.len(), 2);
    assert_eq!(range2.len(), 2);

    vec![range1[0].min(range2[0]), range1[1].max(range2[1])]
}
