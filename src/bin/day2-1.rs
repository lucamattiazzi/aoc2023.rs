use core::str::Lines;
use regex::Regex;
use std::env;
use std::fs;

// 12 red cubes, 13 green cubes, and 14 blue cubes

fn validate_color_number(color: &str, number: u32) -> bool {
    if color == "red" {
        return number <= 12;
    }

    if color == "green" {
        return number <= 13;
    }

    if color == "blue" {
        return number <= 14;
    }

    return true;
}

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

fn check_valid(subsets: Vec<&str>) -> bool {
    for subset in subsets {
        for block in subset.split(",") {
            let color = get_color(block);
            let re = Regex::new(r"[^0-9]*").unwrap();
            let number = re.replace_all(block, "");
            let number = number.parse::<u32>().unwrap();
            let is_valid = validate_color_number(color.unwrap(), number);
            if !is_valid {
                return false;
            }
        }
    }
    return true;
}

fn main() {
    let file_path: String = env::args().nth(1).expect("Please supply a file path");
    let file_content: String = fs::read_to_string(file_path).unwrap();
    let lines: Lines = file_content.lines();

    let mut total_value = 0;
    for (index, line) in lines.enumerate() {
        let games: Vec<&str> = line.split(":").last().unwrap().split(";").collect();
        let mut game_possible = true;
        for game in games {
            let subsets: Vec<&str> = game.split(",").collect();
            let is_valid = check_valid(subsets);
            if !is_valid {
                game_possible = false;
                break;
            }
        }
        if game_possible {
            total_value += index + 1;
        }
    }
    println!("Total value: {}", total_value)
}
