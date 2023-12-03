use core::str::Lines;
use std::char;
use std::env;
use std::fs;

fn get_all_numbers(string: String) -> Vec<u32> {
    let mut all_numbers: Vec<u32> = Vec::new();
    let chars: Vec<char> = string.chars().collect();
    for character in chars {
        if character.is_numeric() {
            let number = character.to_digit(10).unwrap() as u32;
            all_numbers.push(number);
        }
    }
    return all_numbers;
}

fn main() {
    let file_path: String = env::args().nth(1).expect("Please supply a file path");
    println!("Using file {}", file_path);

    let file_content: String = fs::read_to_string(file_path).unwrap();
    let lines: Lines = file_content.lines();

    let mut total_sum: u32 = 0;
    for line in lines {
        let all_numbers: Vec<u32> = get_all_numbers(line.to_string());
        let first: u32 = *all_numbers.first().unwrap() as u32;
        let last: u32 = *all_numbers.last().unwrap() as u32;
        let value: u32 = first * 10 + last;
        println!("From {:?} got {:?}", line, value);
        total_sum += value;
    }
    println!("{:?}", total_sum)
}
