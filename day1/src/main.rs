use std::collections::HashMap;
use std::fs::read_to_string;
use std::num::ParseIntError;
use std::str::FromStr;

use regex::{Captures, Match, Regex};

fn main() {
    part2();
}



fn to_digit(s: &str, str_to_u32: &HashMap<&str, u32> ) -> Result<u32, ParseIntError>{
    return if s.len() == 1 {
        u32::from_str(s)
    } else {
        if let Some(digit) = str_to_u32.get(s) {
            Ok(*digit)
        } else {
            panic!();
        }
    }
}


fn part2(){
    let file_path = "part1_input.txt";
    let contents_input = read_to_string(file_path).unwrap();
    let mut sum = 0;
    let digit_name_to_digits: HashMap<&str, u32> =  [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ].iter().cloned().collect();
    let reversed_digit_name_to_digits: HashMap<&str, u32> = [
        ("eno", 1),
        ("owt", 2),
        ("eerht", 3),
        ("ruof", 4),
        ("evif", 5),
        ("xis", 6),
        ("neves", 7),
        ("thgie", 8),
        ("enin", 9)
    ].iter().cloned().collect();
    let first_match_regexp = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let last_match_regexp = Regex::new(r"\d|enin|thgie|neves|xis|evif|ruof|eerht|owt|eno").unwrap();
    for line in contents_input.lines(){
        let first_digits: Captures = first_match_regexp.captures(line).unwrap();
        let first_match: Match = first_digits.iter().next().unwrap().unwrap();
        let first_digit = to_digit(first_match.as_str(), &digit_name_to_digits).unwrap();

        let reversed_line_string = line.chars().rev().collect::<String>();
        let reversed_line: &str = reversed_line_string.as_str();
        let last_digits: Captures = last_match_regexp.captures(reversed_line).unwrap();
        let last_match: Match = last_digits.iter().next().unwrap().unwrap();
        let last_digit = to_digit(last_match.as_str(), &reversed_digit_name_to_digits).unwrap();

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
