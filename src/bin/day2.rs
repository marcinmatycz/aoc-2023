use std::fs::read_to_string;

fn validate_number_of_cubes(number: &str, color: &str) -> bool {
    let number = u32::from_str_radix(number, 10).unwrap();
    if number > 14 {
        return false;
    }

    if color == "green" && number > 13 {
        return false;
    }

    if color == "red" && number > 12 {
        return false;
    }

    true
}

fn is_game_possible(line: &str) -> u32 {
    const GAME_ID_TOKEN_INDEX: usize = 1;
    let tokens: Vec<&str> = line.split([' ', ':', ',', ';']).collect();
    let is_possible = (GAME_ID_TOKEN_INDEX + 1..tokens.len())
        .filter(|&i| u32::from_str_radix(tokens[i], 10).unwrap_or(0) >= 12)
        .map(|index| validate_number_of_cubes(tokens[index], tokens[index + 1]))
        .reduce(|a, b| a && b);

    match is_possible {
        None | Some(true) => u32::from_str_radix(tokens[GAME_ID_TOKEN_INDEX], 10).unwrap(),
        Some(false) => 0,
    }
}

fn get_max_color_value(draw_pairs: &Vec<Vec<&str>>, key: &str) -> u32 {
    const COLOR_KEY_INDEX: usize = 1;
    const COLOR_VALUE_INDEX: usize = 0;
    draw_pairs
        .iter()
        .filter(|pair| pair[COLOR_KEY_INDEX] == key)
        .map(|pair| u32::from_str_radix(pair[COLOR_VALUE_INDEX], 10).unwrap())
        .max()
        .unwrap()
}

fn get_power_of_set(line: &str) -> u32 {
    let tokens = line.split([':', ',', ';']).collect::<Vec<&str>>();
    const COLOR_KEY_INDEX: usize = 1;
    let mut draw_pairs: Vec<_> = tokens
        .iter()
        .skip(1)
        .map(|token| token[1..].split(' ').collect::<Vec<_>>())
        .collect();

    draw_pairs.sort_by(|a, b| a[COLOR_KEY_INDEX].cmp(b[COLOR_KEY_INDEX]));

    let max_blues = get_max_color_value(&draw_pairs, "blue");
    let max_reds = get_max_color_value(&draw_pairs, "red");
    let max_greens = get_max_color_value(&draw_pairs, "green");
    max_blues * max_reds * max_greens
}

fn calculate_first_part(input: &str) -> u32 {
    input.lines().map(is_game_possible).sum()
}

fn calculate_second_part(input: &str) -> u32 {
    input.lines().map(get_power_of_set).sum()
}

fn main() {
    let input = read_to_string("/home/marcinmatycz/repos/aoc-2023/inputs/day2_input").unwrap();
    println!("{}", calculate_first_part(&input));
    println!("{}", calculate_second_part(&input));
}
