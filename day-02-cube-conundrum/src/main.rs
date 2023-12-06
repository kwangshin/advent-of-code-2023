use std::fs::read_to_string;

fn main() {
    println!("Advent of Code 2023 - Day 02 Cube Conundrum");

    let mut sum_of_id_for_possible_games: u32 = 0;

    for input_line in read_to_string("./resource/input.txt").unwrap().lines() {
        sum_of_id_for_possible_games += get_game_id_if_possible_game(input_line);
    }

    println!("The sum of the IDs of possible games is {}.", sum_of_id_for_possible_games);

    let mut sum_of_power_of_minimum_set_of_cubes: u32 = 0;

    for input_line in read_to_string("./resource/input.txt").unwrap().lines() {
        sum_of_power_of_minimum_set_of_cubes += get_power_of_set_of_cubes(input_line);
    }

    println!("The sum of the power of the minimum set of cubes is {}.", sum_of_power_of_minimum_set_of_cubes)
}

fn get_game_id_if_possible_game(input_line: &str) -> u32 {
    let max_number_of_red_cubes = 12;
    let max_number_of_green_cubes = 13;
    let max_number_of_blue_cubes = 14;
    
    let game_number = input_line.split(":").next().unwrap().split_whitespace().last().unwrap().parse::<u32>().unwrap();

    let game_string = input_line.split(":").last().unwrap();

    for each_game in game_string.split(";") {
        for each_cube in each_game.split(",") {
            let number_of_cubes = each_cube.split_whitespace().next().unwrap().parse::<u32>().unwrap();
            let color_of_cubes = each_cube.split_whitespace().last().unwrap();

            if color_of_cubes == "red" {
                if number_of_cubes > max_number_of_red_cubes {
                    return 0;
                }
            } else if color_of_cubes == "green" {
                if number_of_cubes > max_number_of_green_cubes {
                    return 0;
                }
            } else if color_of_cubes == "blue" {
                if number_of_cubes > max_number_of_blue_cubes {
                    return 0;
                }
            }
        }
    }

    return game_number;
}

fn get_power_of_set_of_cubes(input_line: &str) -> u32 {
    let mut max_number_of_red_cubes = 1;
    let mut max_number_of_green_cubes = 1;
    let mut max_number_of_blue_cubes = 1;

    let game_string = input_line.split(":").last().unwrap();

    for each_game in game_string.split(";") {
        for each_cube in each_game.split(",") {
            let number_of_cubes = each_cube.split_whitespace().next().unwrap().parse::<u32>().unwrap();
            let color_of_cubes = each_cube.split_whitespace().last().unwrap();

            if color_of_cubes == "red" {
                if number_of_cubes > max_number_of_red_cubes {
                    max_number_of_red_cubes = number_of_cubes;
                }
            } else if color_of_cubes == "green" {
                if number_of_cubes > max_number_of_green_cubes {
                    max_number_of_green_cubes = number_of_cubes;
                }
            } else if color_of_cubes == "blue" {
                if number_of_cubes > max_number_of_blue_cubes {
                    max_number_of_blue_cubes = number_of_cubes;
                }
            }
        }
    }
    
    return max_number_of_red_cubes * max_number_of_green_cubes * max_number_of_blue_cubes;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_game_id_if_possible_game_1() {
        assert_eq!(get_game_id_if_possible_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"), 1);
    }

    #[test]
    fn test_get_game_id_if_possible_game_2() {
        assert_eq!(get_game_id_if_possible_game("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"), 2);
    }

    #[test]
    fn test_get_game_id_if_possible_game_3() {
        assert_eq!(get_game_id_if_possible_game("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"), 0);
    }

    #[test]
    fn test_get_game_id_if_possible_game_4() {
        assert_eq!(get_game_id_if_possible_game("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"), 0);
    }

    #[test]
    fn test_get_game_id_if_possible_game_5() {
        assert_eq!(get_game_id_if_possible_game("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"), 5);
    }

    #[test]
    fn test_get_power_of_set_of_cubes_1() {
        assert_eq!(get_power_of_set_of_cubes("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"), 48);
    }

    #[test]
    fn test_get_power_of_set_of_cubes_2() {
        assert_eq!(get_power_of_set_of_cubes("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"), 12);
    }

    #[test]
    fn test_get_power_of_set_of_cubes_3() {
        assert_eq!(get_power_of_set_of_cubes("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"), 1560);
    }

    #[test]
    fn test_get_power_of_set_of_cubes_4() {
        assert_eq!(get_power_of_set_of_cubes("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"), 630);
    }

    #[test]
    fn test_get_power_of_set_of_cubes_5() {
        assert_eq!(get_power_of_set_of_cubes("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"), 36);
    }
    
}