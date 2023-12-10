use std::char;
use std::fs::read_to_string;

fn main() {
    const LIST_OF_SPELLED_DIGITS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut sum_of_calibration_values: u64 = 0;

    for line in read_to_string("/home/marcinmatycz/repos/aoc-2023/inputs/day1_input")
        .unwrap()
        .lines()
    {
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
    }
    println!("{}", sum_of_calibration_values);
}
