use std::fs::read_to_string;

fn find_calibration_value(line: &str) -> u32 {
    let first_digit = line.chars().find_map(|c| c.to_digit(10)).unwrap();
    let second_digit = line.chars().rev().find_map(|c| c.to_digit(10)).unwrap();
    first_digit * 10 + second_digit
}

fn find_calibration_value2(line: &str) -> u32 {
    const SPELLED_DIGITS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let digits_representations = SPELLED_DIGITS.iter().enumerate()
        .map(|(index, spelled_digit)| {
            let digit_value = u32::try_from(index + 1).unwrap();
            let char_digit = char::from_digit(digit_value, 10).unwrap();
            (digit_value, spelled_digit, char_digit)

        });

    let first_digit = digits_representations.clone()
        .map(|(digit_value, spelled_digit, char_digit)| {
            let spelled_index = line.find(spelled_digit).unwrap_or(line.len());
            let digit_index = line.find(char_digit).unwrap_or(line.len());
            (spelled_index.min(digit_index), digit_value)
        })
        .min_by_key(|pair| pair.0)
        .unwrap()
        .1;

    let last_digit = digits_representations 
        .map(|(digit_value, spelled_digit, char_digit)| {
            let spelled_index = line
                .rfind(spelled_digit)
                .map_or_else(|| -1, |v| i32::try_from(v).unwrap());
            let digit_index = line
                .rfind(char_digit)
                .map_or_else(|| -1, |v| i32::try_from(v).unwrap());
            (spelled_index.max(digit_index), digit_value)
        })
        .max_by_key(|pair| pair.0)
        .unwrap()
        .1;

    first_digit * 10 + last_digit
}

fn calculate_first_part(input: &str) -> u32 {
    input.lines().map(find_calibration_value).sum()
}

fn calculate_second_part(input: &str) -> u32 {
    input.lines().map(find_calibration_value2).sum()
}

fn main() {
    let input = read_to_string("/home/marcinmatycz/repos/aoc-2023/inputs/day1_input").unwrap();
    println!("{}", calculate_first_part(&input));
    println!("{}", calculate_second_part(&input));
}
