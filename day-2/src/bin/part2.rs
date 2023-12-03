use core::str::Lines;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;

fn get_color(block: &str) -> Option<&str> {
    let red_idx: Option<usize> = block.find("red");
    if red_idx != None {
        return Some("red");
    }

    let green_idx: Option<usize> = block.find("green");
    if green_idx != None {
        return Some("green");
    }

    let blue_idx: Option<usize> = block.find("blue");
    if blue_idx != None {
        return Some("blue");
    }

    return None;
}

fn compute_power(line: &str) -> u32 {
    let games: Vec<&str> = line.split(":").last().unwrap().split(";").collect();
    let mut maxes = HashMap::new();
    for game in games {
        let subsets: Vec<&str> = game.split(",").collect();
        for subset in subsets {
            for block in subset.split(",") {
                let color = get_color(block);
                let re = Regex::new(r"[^0-9]*").unwrap();
                let number = re.replace_all(block, "");
                let number = number.parse::<u32>().unwrap();
                let current_max = maxes.get(&color);
                if current_max == None {
                    maxes.insert(color, number);
                } else {
                    let current_max = *current_max.unwrap();
                    if number > current_max {
                        maxes.insert(color, number);
                    }
                }
            }
        }
    }
    let mut total = 1;
    for (_key, value) in maxes.iter() {
        total *= value;
    }
    return total;
}

fn main() {
    let file_path: String = env::args().nth(1).expect("Please supply a file path");
    let file_content: String = fs::read_to_string(file_path).unwrap();
    let lines: Lines = file_content.lines();

    let mut total_value = 0;
    for line in lines {
        let game_power = compute_power(line);
        total_value += game_power;
    }
    println!("Total value: {}", total_value)
}
