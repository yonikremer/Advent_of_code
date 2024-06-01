use std::collections::HashSet;
use std::fs::read_to_string;
use std::cmp::min;

fn part1(file_path: &str) -> usize {
    let contents_input: String = read_to_string(file_path).unwrap();
    let mut sum: usize = 0_usize;
    for line in contents_input.lines() {
        let mut points_in_line: usize = 0_usize;
        for _ in 0..num_winning_numbers_you_have(&line) {
            if points_in_line == 0 {
                points_in_line = 1;
            } else {
                points_in_line *= 2;
            }
        }
        sum += points_in_line;
    }
    sum
}

fn num_winning_numbers_you_have(line: &str) -> usize {
    let colon_index: usize = line.find(':').unwrap();
    let pipe_index: usize = line.find('|').unwrap();
    let winning_numbers_str: &str = &line[colon_index + 1..pipe_index];
    let numbers_you_have: &str = &line[pipe_index + 1..];
    let winning_numbers: HashSet<usize> = winning_numbers_str
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let numbers_you_have: HashSet<usize> = numbers_you_have
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    winning_numbers.intersection(&numbers_you_have).count()
}


fn part2(file_path: &str) -> usize {
    let contents_input: String = read_to_string(file_path).unwrap();
    let num_lines: usize = contents_input.lines().count();
    let mut num_copies: Vec<usize> = vec![1; num_lines];
    for (line_index, line) in contents_input.lines().enumerate() {
        let num_copies_of_line: usize = num_copies[line_index];
        let end: usize = min(line_index + 1 + num_winning_numbers_you_have(line), num_lines + 1);
        for i in line_index + 1..end {
            num_copies[i] += num_copies_of_line;
        }
    }
    return num_copies.iter().sum();
}


fn main() {
    let part_1_result = part1("input.txt");
    println!("{part_1_result}");


    let part_2_result = part2("input.txt");
    println!("{part_2_result}");
}
