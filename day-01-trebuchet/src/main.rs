use std::fs::read_to_string;

fn main() {
    println!("Advent of Code 2023 - Day 01 Trebuchet?!");

    let mut total_calibration_value: u32 = 0;

    for line in read_to_string("./resource/input.txt").unwrap().lines() {
        total_calibration_value += get_calibration_value(line)
    }

    println!("Total calibration value is {}.", total_calibration_value)
}

fn get_calibration_value(input_line: &str) -> u32 {
    let mut calibration_value: u32 = 0;

    let line_length = input_line.len();

    for index in 0..line_length {
        let substring = &input_line[index..];

        let valid_digit: u32 = get_valid_digit_including_letters(substring);

        if valid_digit < 1 {
            // Invlid digit!
        } else {
            calibration_value += valid_digit * 10;
            break;
        }
    }

    for index in 0..line_length {
        let substring = &input_line[line_length-index-1..];

        let valid_digit: u32 = get_valid_digit_including_letters(substring);

        if valid_digit < 1 {
            // Invlid digit!
        } else {
            calibration_value += valid_digit;
            break;
        }
    }

    return calibration_value;
}

fn get_valid_digit_including_letters(input_string: &str) -> u32 {
    let first_character = input_string.chars().next().unwrap();

    if first_character.is_digit(10) {
        return first_character.to_digit(10).unwrap();
    } else if input_string.starts_with("one") {
        return 1;
    } else if input_string.starts_with("two") {
        return 2;
    } else if input_string.starts_with("three") {
        return 3;
    } else if input_string.starts_with("four") {
        return 4;
    } else if input_string.starts_with("five") {
        return 5;
    } else if input_string.starts_with("six") {
        return 6;
    } else if input_string.starts_with("seven") {
        return 7;
    } else if input_string.starts_with("eight") {
        return 8;
    } else if input_string.starts_with("nine") {
        return 9;
    }

    return 0
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_calibration_value_case_1() {
        assert_eq!(get_calibration_value("1abc2"), 12);
    }

    #[test]
    fn test_get_calibration_value_case_2() {
        assert_eq!(get_calibration_value("pqr3stu8vwx"), 38);
    }

    #[test]
    fn test_get_calibration_value_case_3() {
        assert_eq!(get_calibration_value("a1b2c3d4e5f"), 15);
    }

    #[test]
    fn test_get_calibration_value_case_4() {
        assert_eq!(get_calibration_value("treb7uchet"), 77);
    }

    #[test]
    fn test_get_calibration_value_including_letters_case_1() {
        assert_eq!(get_calibration_value("two1nine"), 29);
    }

    #[test]
    fn test_get_calibration_value_including_letters_case_2() {
        assert_eq!(get_calibration_value("eightwothree"), 83);
    }

    #[test]
    fn test_get_calibration_value_including_letters_case_3() {
        assert_eq!(get_calibration_value("abcone2threexyz"), 13);
    }

    #[test]
    fn test_get_calibration_value_including_letters_case_4() {
        assert_eq!(get_calibration_value("xtwone3four"), 24);
    }

    #[test]
    fn test_get_calibration_value_including_letters_case_5() {
        assert_eq!(get_calibration_value("4nineeightseven2"), 42);
    }

    #[test]
    fn test_get_calibration_value_including_letters_case_6() {
        assert_eq!(get_calibration_value("zoneight234"), 14);
    }

    #[test]
    fn test_get_calibration_value_including_letters_case_7() {
        assert_eq!(get_calibration_value("7pqrstsixteen"), 76);
    }
}