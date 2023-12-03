use std::char;
use std::fs::read_to_string;

fn main() {
    let mut sum_of_calibration_values: u64 = 0;

    for line in read_to_string("/home/marcinmatycz/repos/aoc-2023/inputs/day1_input")
        .unwrap()
        .lines()
    {
        let matched: Vec<_> = line.matches(|c: char| c.is_ascii_digit()).collect();
        let first_digit = matched.first().unwrap().parse::<u64>().unwrap();
        let second_digit = matched.last().unwrap().parse::<u64>().unwrap();
        sum_of_calibration_values += first_digit * 10 + second_digit;
    }
    println!("{}", sum_of_calibration_values);
}
