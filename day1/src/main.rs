use std::collections::HashMap;
use std::fs::read_to_string;
use std::str::FromStr;

use regex::{Captures, Regex};

fn main() {
    part1();
    part2();
}


fn part2(){
    let file_path = "part1_input.txt";
    let contents_input = read_to_string(file_path).unwrap();
    let mut sum = 0;
    let digit_name_to_digits: HashMap<&str, u32> =  IntoIterator::into_iter([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ]).collect();
    let reversed_digit_name_to_digits: HashMap<&str, u32> = IntoIterator::into_iter([
        ("eno", 1),
        ("owt", 2),
        ("eerht", 3),
        ("ruof", 4),
        ("evif", 5),
        ("xis", 6),
        ("neves", 7),
        ("thgie", 8),
        ("enin", 9)
    ]).collect();
    let first_match_regexp = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").expect("Invalid regex");
    let last_match_regexp = Regex::new(r"\d|enin|thgie|neves|xis|evif|ruof|eerht|owt|eno").expect("Invalid regex");
    for line in contents_input.lines(){
        let first_digits: Captures = first_match_regexp.captures(line).expect(format!("No digit in line {}", line).as_str());
        let first_match = first_digits.iter().next().unwrap().unwrap().as_str();
        let first_error_message = format!("Regex matched invalid digit {first_match} in line {line}");
        let first_digit = if first_match.len() == 1 {
            u32::from_str(first_match).expect(first_error_message.as_str())
        } else {
            *digit_name_to_digits.get(first_match).expect(first_error_message.as_str())
        };

        let reversed_line_string: String = line.chars().rev().collect();
        let reversed_line: &str = reversed_line_string.as_str();
        let last_digits: Captures = last_match_regexp.captures(reversed_line).expect(format!("No digit in line {}", line).as_str());
        let last_match = last_digits.iter().next().unwrap().unwrap().as_str();
        let last_error_message = format!("Regex matched invalid digit {last_match} in line {line}");
        let last_digit = if last_match.len() == 1 {
            u32::from_str(last_match).expect(last_error_message.as_str())
        } else {
            *reversed_digit_name_to_digits.get(last_match).expect(last_error_message.as_str())
        };
        sum += first_digit * 10 + last_digit;
    }
    println!("{}", sum);
}

fn part1() {
    let file_path = "part1_input.txt";
    let contents_input = read_to_string(file_path).unwrap();
    let mut sum: u32 = 0;
    for line in contents_input.lines() {
        let mut digits = line.chars().filter(|ch: &char| {
            ch.is_digit(10)
        });
        let first_digit = digits.next().expect(format!("Line without a digit {}", line).as_str());
        let mut last_digit = first_digit;
        while let Some(current_digit) = digits.next() {
            last_digit = current_digit;
        }
        let tens: u32 = first_digit.to_digit(10).unwrap();
        let units: u32 = last_digit.to_digit(10).unwrap();
        sum += tens * 10 + units;
    }
    println!("{}", sum);
}
