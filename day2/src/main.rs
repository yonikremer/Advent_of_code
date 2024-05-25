use regex::{Captures, Match, Regex};

use std::fs::read_to_string;


fn part1() {
    let file_path = "part1_example.txt";
    let contents_input = read_to_string(file_path).unwrap();
    let mut game_id_sum_sum: usize = 0;
    let num_green_regex: Regex = Regex::new(r"(\d+) green").expect("Invalid regex");
    let num_blue_regex: Regex = Regex::new(r"(\d+) blue").expect("Invalid regex");
    let num_red_regex: Regex = Regex::new(r"(\d+) red").expect("Invalid regex");
    let max_red: u32 = 12;
    let max_green: u32 = 13;
    let max_blue: u32 = 14;
    for (i, line) in contents_input.lines().enumerate() {

        let num_green_matches: Captures = num_green_regex.captures(line).expect(format!("No num green in line {}", line).as_str());
        let max_num_green_in_line = num_green_matches.iter().map(|curr_match: Option<Match>|{
            if curr_match.is_some() {
                curr_match.unwrap().as_str().parse::<u32>().unwrap()
            } else {
                0
            }
        }).max().unwrap_or(0);

        let num_blue_matches: Captures = num_blue_regex.captures(line).expect(format!("No num blue in line {}", line).as_str());
        let max_num_blue_in_line = num_blue_matches.iter().map(|curr_match: Option<Match>|{
            if curr_match.is_some() {
                curr_match.unwrap().as_str().parse::<u32>().unwrap()
            } else {
                0
            }
        }).max().unwrap_or(0);

        let num_red_matches: Captures = num_red_regex.captures(line).expect(format!("No num red in line {}", line).as_str());
        let max_num_red_in_line = num_red_matches.iter().map(|curr_match: Option<Match>|{
            if curr_match.is_some() {
                curr_match.unwrap().as_str().parse::<u32>().unwrap()
            } else {
                0
            }
        }).max().unwrap_or(0);

        if max_num_green_in_line <= max_green || max_num_blue_in_line <= max_blue || max_num_red_in_line <= max_red {
            game_id_sum_sum += i + 1;
        }
    }
    println!("{}", game_id_sum_sum);
}


fn main() {
    part1();
}
