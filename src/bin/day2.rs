use std::fs::read_to_string;

fn validate_number_of_cubes(number: &str, color: &str) -> bool {
    let number = number.to_string().parse::<u32>().unwrap();
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
        .filter(|&i| tokens[i].to_string().parse::<u32>().unwrap_or(0) >= 12)
        .map(|index| validate_number_of_cubes(tokens[index], tokens[index + 1]))
        .reduce(|a, b| a && b);

    match is_possible {
        None | Some(true) => tokens[GAME_ID_TOKEN_INDEX]
            .to_string()
            .parse::<u32>()
            .unwrap(),
        Some(false) => 0,
    }
}

fn calculate_first_part(input: &str) -> u32 {
    input.lines().map(is_game_possible).sum()
}

fn main() {
    let input = read_to_string("/home/marcinmatycz/repos/aoc-2023/inputs/day2_input").unwrap();
    println!("{}", calculate_first_part(&input));
}
