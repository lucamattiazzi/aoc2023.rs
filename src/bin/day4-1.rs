use core::str::Lines;
// use regex::Regex;
use std::env;
use std::fs;

fn main() {
    let file_path: String = env::args().nth(1).expect("Please supply a file path");
    let file_content: String = fs::read_to_string(file_path).unwrap();
    let lines: Lines = file_content.lines();
    let mut all_winning_numbers: Vec<Vec<i32>> = Vec::new();

    for line in lines {
        let ticket = line.split(": ").last().unwrap();
        let split = ticket.split_once("|");
        if split.is_none() {
            continue;
        }

        let (winners_str, numbers_str) = split.unwrap();
        let winners: Vec<i32> = winners_str
            .split(" ")
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let numbers: Vec<i32> = numbers_str
            .split(" ")
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let winning_numbers: Vec<&i32> = winners.iter().filter(|x| numbers.contains(x)).collect();
        all_winning_numbers.push(winning_numbers.iter().map(|x| **x).collect());
    }
    println!("all_winning_numbers: {:?}", all_winning_numbers);
    let mut total_points = 0;
    let base: i32 = 2;
    for winning_numbers in all_winning_numbers {
        let winners_len: u32 = winning_numbers.len().try_into().unwrap();
        if winners_len == 0 {
            continue;
        }
        total_points += base.pow(winners_len - 1);
    }
    println!("total_points: {:?}", total_points);
}
