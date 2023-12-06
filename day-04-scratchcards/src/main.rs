use std::fs::read_to_string;

fn main() {
    println!("Advent of Code 2023 - Day 04 Scratchcards");

    let mut total_points: u32 = 0;

    let input_file_path = "./resource/input.txt";

    for line in read_to_string(input_file_path).unwrap().lines() {
        total_points += get_points_per_card(line)
    }

    println!("Part Onw : Total point is {}.", total_points);
    println!("Part Two : Total scratchcards is {}.", get_total_number_of_scratchcards_in_part_two(input_file_path));
}

fn get_points_per_card(input_line: &str) -> u32 {
    let mut total_points_per_card: u32 = 0;

    let mut card_numbers = input_line.split(":").last().unwrap().split("|");
    let winning_numbers = card_numbers.next().unwrap();
    
    let my_numbers = card_numbers.next().unwrap();

    let winning_numbers_vector: Vec<String> = winning_numbers.split_whitespace().map(str::to_string).collect();

    let my_numbers_vector: Vec<String> = my_numbers.split_whitespace().map(str::to_string).collect();
    
    for my_one_number in my_numbers_vector.iter() {
        if winning_numbers_vector.iter().any(|e| e == my_one_number) {
            if total_points_per_card == 0 {
                total_points_per_card = 1
            } else {
                total_points_per_card *= 2
            }
        }
    }

    return total_points_per_card;
}

fn get_matching_number_per_card(input_line: &str) -> u32 {
    let mut total_matching_number: u32 = 0;

    let mut card_numbers = input_line.split(":").last().unwrap().split("|");
    let winning_numbers = card_numbers.next().unwrap();
    
    let my_numbers = card_numbers.next().unwrap();

    let winning_numbers_vector: Vec<String> = winning_numbers.split_whitespace().map(str::to_string).collect();

    let my_numbers_vector: Vec<String> = my_numbers.split_whitespace().map(str::to_string).collect();
    
    for my_one_number in my_numbers_vector.iter() {
        if winning_numbers_vector.iter().any(|e| e == my_one_number) {
            total_matching_number += 1
        }
    }

    return total_matching_number;
}

fn get_total_number_of_scratchcards_in_part_two(files_name: &str) -> u32 {
    let mut instance_of_card: Vec<u32> = Vec::new();

    let mut index = 0;
    for line in read_to_string(files_name).unwrap().lines() {
        if index >= instance_of_card.len() {
            instance_of_card.push(1);
        } else {
            instance_of_card[index] += 1;
        }
        
        let matching_number: usize = usize::try_from(get_matching_number_per_card(line)).unwrap();

        if matching_number > 0 {
            for index_of_matching_card in 1..=matching_number {
                
                if (index + index_of_matching_card) >= instance_of_card.len() {
                    instance_of_card.push(instance_of_card[index]);
                } else {
                    instance_of_card[index + index_of_matching_card] += instance_of_card[index];
                }
            }
        }

        index += 1;
    }

    return instance_of_card.iter().sum();
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_points_per_card_1() {
        assert_eq!(get_points_per_card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"), 8);
    }

    #[test]
    fn test_get_points_per_card_2() {
        assert_eq!(get_points_per_card("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"), 2);
    }

    #[test]
    fn test_get_points_per_card_3() {
        assert_eq!(get_points_per_card("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"), 2);
    }

    #[test]
    fn test_get_points_per_card_4() {
        assert_eq!(get_points_per_card("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"), 1);
    }

    #[test]
    fn test_get_points_per_card_5() {
        assert_eq!(get_points_per_card("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"), 0);
    }

    #[test]
    fn test_get_points_per_card_6() {
        assert_eq!(get_points_per_card("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"), 0);
    }

    #[test]
    fn test_get_total_number_of_scratchcards_in_part_two() {
        assert_eq!(get_total_number_of_scratchcards_in_part_two("./resource/part_two_test_input.txt"), 30);
    }
}