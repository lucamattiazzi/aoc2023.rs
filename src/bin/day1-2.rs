use core::str::Lines;
use std::char;
use std::collections::HashMap;
use std::env;
use std::fs;

fn get_all_numbers(string: String) -> Vec<u32> {
    let numbers_literals: HashMap<&str, i32> = HashMap::from([
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let mut all_numbers: Vec<u32> = Vec::new();
    let chars: Vec<char> = string.chars().collect();
    for (index, character) in chars.iter().enumerate() {
        if character.is_numeric() {
            let number = character.to_digit(10).unwrap() as u32;
            all_numbers.push(number);
        } else if character.is_alphabetic() {
            for (key, _value) in numbers_literals.iter() {
                let key_len: usize = key.len();
                let remaining_chars: usize = chars.len() - index;
                if remaining_chars < key_len {
                    continue;
                }
                let slice: String = chars[index..key_len + index].iter().collect();
                if key == &slice {
                    let value: i32 = *numbers_literals.get(key).unwrap();
                    all_numbers.push(value as u32);
                }
            }
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
