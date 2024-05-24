use std::fs::read_to_string;

fn main() {
    part1();
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
