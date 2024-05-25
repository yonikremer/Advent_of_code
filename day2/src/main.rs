use std::fs::read_to_string;

use regex::{CaptureMatches, Captures, Regex};


const NUM_GREEN_REGEX: Regex = Regex::new(r"(\d+) green").expect("Invalid regex");
const NUM_BLUE_REGEX: Regex = Regex::new(r"(\d+) blue").expect("Invalid regex");
const NUM_RED_REGEX: Regex = Regex::new(r"(\d+) red").expect("Invalid regex");


fn part1() {
    let file_path = "input.txt";
    let contents_input = read_to_string(file_path).unwrap();
    let mut game_id_sum_sum: usize = 0;
    let max_red: u32 = 12;
    let max_green: u32 = 13;
    let max_blue: u32 = 14;
    for (i, line) in contents_input.lines().enumerate() {
        let num_green = max_uint_match(&NUM_GREEN_REGEX, line);
        if num_green > max_green { continue; }
        let num_blue = max_uint_match(&NUM_BLUE_REGEX, line);
        if num_blue > max_blue { continue; }
        let num_red = max_uint_match(&NUM_RED_REGEX, line);
        if num_red > max_red { continue; }
        game_id_sum_sum += i + 1;
    }
    println!("{}", game_id_sum_sum);
}


fn part2() {
    let file_path = "input.txt";
    let contents_input = read_to_string(file_path).unwrap();
    let mut power_set_sum: u32 = 0;
    for line in contents_input.lines() {
        let num_green = max_uint_match(&NUM_GREEN_REGEX, line);
        let num_blue = max_uint_match(&NUM_BLUE_REGEX, line);
        let num_red = max_uint_match(&NUM_RED_REGEX, line);
        power_set_sum += num_green * num_blue * num_red;
    }
    println!("{}", power_set_sum);
}


fn max_uint_match(regex: &Regex, line: &str) -> u32 {
    // Given a regex with one capture group, and a string, return the max value
    // in the string that matches the regex
    let green_matches: CaptureMatches = regex.captures_iter(line);
    green_matches.map(|curr_match: Captures| {
        curr_match
            .get(1)
            .expect("Not found green")
            .as_str().parse::<u32>()
            .expect("Regex matched non-integer value")
    }).max().unwrap_or(0)
}


fn main() {
    part2();
}
