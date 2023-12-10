use std::fs::read_to_string;
use std::collections::HashMap;

fn find_calibration_value(line: &str) -> u32 {
    let first_digit = line.chars().find_map(|c| c.to_digit(10)).unwrap();
    let second_digit = line.chars().rev().find_map(|c| c.to_digit(10)).unwrap();
    first_digit * 10 + second_digit
}

fn find_calibration_value2(line: &str) -> u32 {
    const LIST_OF_SPELLED_DIGITS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
        /*
    let spelled_digits_map = HashMap::from([
                                ("one", "1"),
                                ("two", "2"),
                                ("three", "3"),
                                ("four", "4"),
                                ("five", "5"),
                                ("six", "6"),
                                ("seven", "7"),
                                ("eight", "8"),
                                ("nine", "9")
    ]);
    */
    println!("{line}");

    let mut line_with_no_spelled_digits = String::from(line);
    for (index, spelled_digit) in LIST_OF_SPELLED_DIGITS.iter().enumerate() {
        let digit = index + 1;
        line_with_no_spelled_digits = line_with_no_spelled_digits.replace(spelled_digit, &digit.to_string());
    }
    println!("{line_with_no_spelled_digits}");
    /*

    let first_spelled_digit = LIST_OF_SPELLED_DIGITS.iter().find_map(|spelled_digit| line.find(spelled_digit));

    let first_digit = line.chars().find_map(|c| c.to_digit(10)).unwrap();
    let second_digit = line.chars().rev().find_map(|c| c.to_digit(10)).unwrap();
    first_digit * 10 + second_digit
    */
    find_calibration_value(&line_with_no_spelled_digits)
}

fn calculate_first_part(input: &str) -> u32 {
    input.lines().map(|line| find_calibration_value(line)).sum()
}

fn calculate_second_part(input: &str) -> u32 {
    input.lines().map(|line| find_calibration_value2(line)).sum()
}
/*
    const LIST_OF_SPELLED_DIGITS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut sum_of_calibration_values: u64 = 0;

        let matched: Vec<_> = line.match_indices(|c: char| c.is_ascii_digit()).collect();
        let mut first_digit = matched.first().unwrap().1.parse::<u64>().unwrap();
        let mut second_digit = matched.last().unwrap().1.parse::<u64>().unwrap();

        let mut first_index = matched.first().unwrap().0;
        let mut last_index = matched.last().unwrap().0;
        for (i, spelled_digit) in LIST_OF_SPELLED_DIGITS.iter().enumerate() {
            if line.contains(spelled_digit) {
                let index_in_string = line.find(spelled_digit).unwrap();
                let rindex_in_string = line.rfind(spelled_digit).unwrap();

                if index_in_string < first_index {
                    first_index = index_in_string;
                    first_digit = i as u64 + 1;
                }

                if rindex_in_string > last_index {
                    last_index = rindex_in_string;
                    second_digit = i as u64 + 1;
                }
            }
        }
        sum_of_calibration_values += first_digit * 10 + second_digit;
    */

fn main() {
    let input = read_to_string("/home/marcinmatycz/repos/aoc-2023/inputs/day1_input").unwrap();
    println!("{}", calculate_first_part(&input));
    println!("{}", calculate_second_part(&input));
}
