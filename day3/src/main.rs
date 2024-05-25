use std::fs::read_to_string;
use std::cmp::min;


#[derive(Debug)]
struct NumberFound {
    line_index: usize,
    start_x: usize,
    end_x: usize,
    value: usize,
}


fn part1(){
    let file_path = "input.txt";
    let contents_input = read_to_string(file_path).unwrap();
    let lines: Vec<&str> = contents_input.lines().collect();
    let mut number_positions: Vec<NumberFound> = Vec::new();
    let mut symbol_grid: Vec<Vec<bool>> = Vec::new();
    // A grid where grid[i, j] is true if (i, j) is a symbol (not a digit/dot)
    let mut sum: usize = 0;
    for (line_index, curr_line) in lines.iter().enumerate() {
        let mut curr_number_start: Option<usize> = None;
        let mut symbol_line: Vec<bool> = Vec::new();
        for (char_index, ch) in curr_line.chars().enumerate() {
            let char_is_digit = ch.is_digit(10);
            symbol_line.push(!char_is_digit && ch != '.');
            if curr_number_start.is_none() && char_is_digit {
                curr_number_start = Some(char_index);
            }
            if curr_number_start.is_some() && !char_is_digit {
                let curr_number = curr_line[curr_number_start.unwrap()..char_index].parse::<usize>().unwrap();
                let found = NumberFound { line_index, start_x: curr_number_start.unwrap(), end_x: char_index, value: curr_number};
                number_positions.push(found);
                curr_number_start = None;
            }
        }
        symbol_grid.push(symbol_line);
    }

    for number in number_positions.iter() {
        let mut has_adjacent_symbols: bool = false;
        let mut lines_to_search: Vec<usize> = Vec::new();
        if number.line_index > 0 {
            lines_to_search.push(number.line_index - 1);
        }
        if number.line_index + 1 < symbol_grid.len() {
            lines_to_search.push(number.line_index + 1);
        }
        let start: usize = if number.start_x == 0 { 0 } else { number.start_x - 1 };
        let end = min(number.end_x + 1, symbol_grid[number.line_index].len());

        for line_to_search in lines_to_search.iter() {
            let symbol_line: &Vec<bool> = &symbol_grid[*line_to_search];
            // Check if there are any adjacent symbols
            for i in start..end {
                if symbol_line[i] {
                    has_adjacent_symbols = true;
                }
            }
        }
        let number_line: &Vec<bool> = &symbol_grid[number.line_index];
        if number.start_x > 0 && number_line[number.start_x - 1] {
            has_adjacent_symbols = true;
        }
        if number.end_x < number_line.len() && number_line[number.end_x] {
            has_adjacent_symbols = true;
        }
        if has_adjacent_symbols {
            sum += number.value;
        } else {
            println!("{:?} Has no adjacent symbols", number);
        }

    }
    println!("The sum of all the numbers is {}", sum);
}


fn main() {
    part1();
}
