use std::fs::read_to_string;
use std::cmp::Ordering;

fn main() {
    println!("Advent of Code 2023 - Day 07 Camel Cards");

    println!("The total winnings is {}.", get_total_winnings("./resource/input.txt", false));
    println!("The total winnings with new joker rule is {}.", get_total_winnings("./resource/input.txt", true));
}

fn get_total_winnings(file_name: &str, with_joker_cards: bool) -> usize {
    let input_string = read_to_string(file_name).unwrap();
    
    get_total_winnings_from_string(input_string.as_str(), with_joker_cards)
}

fn get_total_winnings_from_string(input_string: &str, with_joker_cards: bool) -> usize {
    let mut total_winnings = 0;

    let mut type_1_vector: Vec<&str> = Vec::new();
    let mut type_2_vector: Vec<&str> = Vec::new();
    let mut type_3_vector: Vec<&str> = Vec::new();
    let mut type_4_vector: Vec<&str> = Vec::new();
    let mut type_5_vector: Vec<&str> = Vec::new();
    let mut type_6_vector: Vec<&str> = Vec::new();
    let mut type_7_vector: Vec<&str> = Vec::new();

    for one_line in input_string.lines() {
        let mut hands: &str = one_line.split_whitespace().next().unwrap();
        
        let hands_char_vector: Vec<char> = hands.chars().collect();
        let number_of_joker_card = hands_char_vector.iter().filter(|&c| *c == 'J').count();
        let hands_without_capital_j: String;

        if with_joker_cards {
            hands_without_capital_j = remove_j_characters(hands);
            hands = hands_without_capital_j.as_str();
        }

        if is_type_1_vector(hands) {
            type_1_vector.push(one_line);
        } else if is_type_2_vector(hands, with_joker_cards, number_of_joker_card) {
            type_2_vector.push(one_line);
        } else if is_type_3_vector(hands) {
            type_3_vector.push(one_line);
        } else if is_type_4_vector(hands, with_joker_cards, number_of_joker_card) {
            type_4_vector.push(one_line);
        } else if is_type_5_vector(hands) {
            type_5_vector.push(one_line);
        } else if is_type_6_vector(hands) {
            type_6_vector.push(one_line);
        } else {
            type_7_vector.push(one_line);
        }
    }
    
    type_1_vector.sort_by(|a, b| compare_cards_with_strength(a, b, with_joker_cards));
    type_2_vector.sort_by(|a, b| compare_cards_with_strength(a, b, with_joker_cards));
    type_3_vector.sort_by(|a, b| compare_cards_with_strength(a, b, with_joker_cards));
    type_4_vector.sort_by(|a, b| compare_cards_with_strength(a, b, with_joker_cards));
    type_5_vector.sort_by(|a, b| compare_cards_with_strength(a, b, with_joker_cards));
    type_6_vector.sort_by(|a, b| compare_cards_with_strength(a, b, with_joker_cards));
    type_7_vector.sort_by(|a, b| compare_cards_with_strength(a, b, with_joker_cards));

    let mut rank: usize = input_string.lines().count();

    for one_hand in type_1_vector {
        let bid_value = one_hand.split_whitespace().last().unwrap().parse::<usize>().unwrap();
        total_winnings += bid_value * rank;
        rank -= 1;
    }
    for one_hand in type_2_vector {
        let bid_value = one_hand.split_whitespace().last().unwrap().parse::<usize>().unwrap();
        total_winnings += bid_value * rank;
        rank -= 1;
    }
    for one_hand in type_3_vector {
        let bid_value = one_hand.split_whitespace().last().unwrap().parse::<usize>().unwrap();
        total_winnings += bid_value * rank;
        rank -= 1;
    }
    for one_hand in type_4_vector {
        let bid_value = one_hand.split_whitespace().last().unwrap().parse::<usize>().unwrap();
        total_winnings += bid_value * rank;
        rank -= 1;
    }
    for one_hand in type_5_vector {
        let bid_value = one_hand.split_whitespace().last().unwrap().parse::<usize>().unwrap();
        total_winnings += bid_value * rank;
        rank -= 1;
    }
    for one_hand in type_6_vector {
        let bid_value = one_hand.split_whitespace().last().unwrap().parse::<usize>().unwrap();
        total_winnings += bid_value * rank;
        rank -= 1;
    }
    for one_hand in type_7_vector {
        let bid_value = one_hand.split_whitespace().last().unwrap().parse::<usize>().unwrap();
        total_winnings += bid_value * rank;
        rank -= 1;
    }

    return total_winnings;
}

fn compare_cards_with_strength(self_value: &str, other_value: &str, with_joker_cards: bool) -> Ordering {
    // A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2.
    let slef_value_vec: Vec<char> = self_value.chars().collect();
    let other_value_vec: Vec<char> = other_value.chars().collect();

    for index in 0..5 {
        
        if slef_value_vec[index] == other_value_vec[index] {
            continue;
        } else {
            if slef_value_vec[index] == 'A' {
                return Ordering::Less;
            } else if other_value_vec[index] == 'A' {
                return Ordering::Greater;
            } else if slef_value_vec[index] == 'K' {
                return Ordering::Less;
            } else if other_value_vec[index] == 'K' {
                return Ordering::Greater;
            } else if slef_value_vec[index] == 'Q' {
                return Ordering::Less;
            } else if other_value_vec[index] == 'Q' {
                return Ordering::Greater;
            } else if slef_value_vec[index] == 'J' {
                if with_joker_cards {
                    return Ordering::Greater;
                } else {
                    return Ordering::Less;
                }
            } else if other_value_vec[index] == 'J' {
                if with_joker_cards {
                    return Ordering::Less;
                } else {
                    return Ordering::Greater;
                }
            } else if slef_value_vec[index] == 'T' {
                return Ordering::Less;
            } else if other_value_vec[index] == 'T' {
                return Ordering::Greater;
            }
            return other_value_vec[index].cmp(&slef_value_vec[index]);
        }
    }

    return Ordering::Equal;
}

fn remove_j_characters(input: &str) -> String {
    let result = input.replace("J", "");
    result
}

fn is_type_1_vector(hands: &str) -> bool {
    let mut hands_char_vector: Vec<char> = hands.chars().collect();
    hands_char_vector.dedup();

    (hands_char_vector.len() == 0) || (hands_char_vector.len() == 1)
}

fn is_type_2_vector(hands: &str, with_joker_cards: bool, number_of_joker_card: usize) -> bool {
    let mut hands_char_vector: Vec<char> = hands.chars().collect();
    let number_of_first_card = hands_char_vector.iter().filter(|&c| *c == hands_char_vector[0]).count();

    let mut has_three_same_number = false;
    for one_char in hands_char_vector.iter() {
        if hands_char_vector.iter().filter(|&c| c == one_char).count() == 3 {
            has_three_same_number = true;
            break;
        }
    }

    hands_char_vector.sort_unstable();
    hands_char_vector.dedup();

    if with_joker_cards {
        if hands_char_vector.len() == 2 {
            if number_of_joker_card > 1 {
                true
            } else if number_of_joker_card == 1 {
                has_three_same_number
            } else if number_of_joker_card == 0 {
                (hands_char_vector.len() == 2) && (number_of_first_card == 1 || number_of_first_card == 4)
            } else {
                false
            }
        } else {
            false
        }
    } else {
        (hands_char_vector.len() == 2) && (number_of_first_card == 1 || number_of_first_card == 4)
    }
}

fn is_type_3_vector(hands: &str) -> bool {
    let mut hands_char_vector: Vec<char> = hands.chars().collect();
    hands_char_vector.sort_unstable();
    hands_char_vector.dedup();

    hands_char_vector.len() == 2
}

fn is_type_4_vector(hands: &str, with_joker_cards: bool, number_of_joker_card: usize) -> bool {
    let mut hands_char_vector: Vec<char> = hands.chars().collect();

    let mut is_type_4 = false;
    for one_char in hands_char_vector.iter() {
        if hands_char_vector.iter().filter(|&c| c == one_char).count() == 3 {
            is_type_4 = true;
            break;
        }
    }

    hands_char_vector.sort_unstable();
    hands_char_vector.dedup();

    if with_joker_cards {
        (hands_char_vector.len() == 3) && (is_type_4 || number_of_joker_card == 2 || number_of_joker_card == 1)
    } else {
        (hands_char_vector.len() == 3) && (is_type_4)
    }
}

fn is_type_5_vector(hands: &str) -> bool {
    let mut hands_char_vector: Vec<char> = hands.chars().collect();
    hands_char_vector.sort_unstable();
    hands_char_vector.dedup();

    hands_char_vector.len() == 3
}

fn is_type_6_vector(hands: &str) -> bool {
    let mut hands_char_vector: Vec<char> = hands.chars().collect();
    hands_char_vector.sort_unstable();
    hands_char_vector.dedup();

    hands_char_vector.len() == 4
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_total_winnings_using_test_input_file() {
        assert_eq!(get_total_winnings("./resource/test_input.txt", false), 6440);

    }

    #[test]
    fn test_get_total_winnings_using_test_input_file_with_joker_cards() {
        assert_eq!(get_total_winnings("./resource/test_input.txt", true), 5905);

    }

    #[test]
    fn test_get_total_winnings_scenario_1() {
        assert_eq!(get_total_winnings_from_string("95999 711\n76676 336\n73777 788\n55T5T 354\n22299 746", false), 9169);
    }

    #[test]
    fn test_get_total_winnings_with_joker_cards_scenario_1() {
        assert_eq!(get_total_winnings_from_string("JJJJJ 1\nJJJJ8 2\n8JJJJ 3\nJJ8JJ 4", true), 29);
    }

    #[test]
    fn test_get_total_winnings_with_joker_cards_scenario_2() {
        assert_eq!(get_total_winnings_from_string("JJJ77 1\nJJJ87 2\nJJJ78 3\n77JJJ 4\n87JJJ 5\n78JJJ 6\nJ77JJ 7\nJ78JJ 8\nJ87JJ 9", true), 226);
    }

    #[test]
    fn test_get_total_winnings_with_joker_cards_scenario_3() {
        assert_eq!(get_total_winnings_from_string("JJ777 1\nJJ778 2\nJJ788 3\nJJ878 4\nJJ789 5\n878JJ 6\n788JJ 7\n778JJ 8\nJ878J 9\nJ788J 10\nJ778J 11", true), 410);
    }

    #[test]
    fn test_get_total_winnings_with_joker_cards_scenario_4() {
        assert_eq!(get_total_winnings_from_string("J6666 1\nJ6667 2\nJ6677 3\nJ6777 4\nJ7777 5\nJ6789 6\nJ6678 7\n56789 8\n66J66 9\n66J67 10\n66J77 11\n67J77 12\n77J77 13\n67J89 14\n66J78 15", true), 942);
    }

    #[test]
    fn test_get_total_winnings_with_joker_cards_scenario_5() {
        assert_eq!(get_total_winnings_from_string("95999 711\n76676 336\n73777 788\n55T5T 354\n22299 746", true), 9169);
    }

    #[test]
    fn test_get_total_winnings_with_joker_cards_scenario_6() {
        assert_eq!(get_total_winnings_from_string("KJKKK 725\n99J59 70\n73777 788\n5892A 641\nJ8JJJ 26\n76676 336\n55T5T 354\n5755T 431\n22299 746\n82462 803\n3J63J 408\n95999 711\nQ59K6 511\nQKJK4 977", true), 51024);
    }
}