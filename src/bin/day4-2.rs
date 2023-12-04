use core::str::Lines;
use std::env;
use std::fs;

struct WinningCount {
    winning_numbers: i32,
    copies: i32,
}

fn main() {
    let file_path: String = env::args().nth(1).expect("Please supply a file path");
    let file_content: String = fs::read_to_string(file_path).unwrap();
    let lines: Lines = file_content.lines();
    let mut winning_counts: Vec<WinningCount> = Vec::new();

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
        let winning_numbers: i32 = winners
            .iter()
            .filter(|x| numbers.contains(x))
            .collect::<Vec<&i32>>()
            .len()
            .try_into()
            .unwrap();
        let results = WinningCount {
            winning_numbers: winning_numbers,
            copies: 1,
        };
        winning_counts.push(results);
    }
    let mut total = 0;
    for i in 0..winning_counts.len() {
        println!(
            "step: {:?} - winners: {} - copies: {}",
            i, winning_counts[i].winning_numbers, winning_counts[i].copies
        );
        for n in 1..winning_counts[i].winning_numbers + 1 {
            let step = i + n as usize;
            winning_counts[step].copies += winning_counts[i].copies;
        }
        total += winning_counts[i].copies;
    }
    println!("total: {:?}", total);
}
