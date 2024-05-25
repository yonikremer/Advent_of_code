use std::fs::read_to_string;

use regex::{CaptureMatches, Captures, Regex};

fn part1() {
    let file_path = "part1_input.txt";
    let contents_input = read_to_string(file_path).unwrap();
    let mut game_id_sum_sum: usize = 0;
    let num_green_regex: Regex = Regex::new(r"(\d+) green").expect("Invalid regex");
    let num_blue_regex: Regex = Regex::new(r"(\d+) blue").expect("Invalid regex");
    let num_red_regex: Regex = Regex::new(r"(\d+) red").expect("Invalid regex");
    let max_red: u32 = 12;
    let max_green: u32 = 13;
    let max_blue: u32 = 14;
    for (i, line) in contents_input.lines().enumerate() {
        let num_green = max_uint_match(&num_green_regex, line);
        if num_green > max_green { continue; }
        let num_blue = max_uint_match(&num_blue_regex, line);
        if num_blue > max_blue { continue; }
        let num_red = max_uint_match(&num_red_regex, line);
        if num_red > max_red { continue; }
        game_id_sum_sum += i + 1;
    }
    println!("{}", game_id_sum_sum);
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
    part1();
}
