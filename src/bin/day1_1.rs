use std::fs::read_to_string;

fn find_calibration_value(line: &str) -> u32 {
    let first_digit = line.chars().find_map(|c| c.to_digit(10)).unwrap();
    let second_digit = line.chars().rev().find_map(|c| c.to_digit(10)).unwrap();
    first_digit * 10 + second_digit
}

fn main() {
    let input = read_to_string("/home/marcinmatycz/repos/aoc-2023/inputs/day1_input").unwrap();
    let sum_of_calibration_values: u32 =
        input.lines().map(|line| find_calibration_value(line)).sum();
    println!("{sum_of_calibration_values}");
}
