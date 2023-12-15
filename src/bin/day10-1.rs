use std::env;
use std::fs;

fn find_next_position(
    map: &Vec<Vec<&str>>,
    current_symbol: &str,
    current_position: (usize, usize),
    previous_position: (usize, usize),
) -> (usize, usize) {
    let above_position = (current_position.0, current_position.1 - 1);
    let below_position = (current_position.0, current_position.1 + 1);
    let left_position = (current_position.0 - 1, current_position.1);
    let right_position = (current_position.0 + 1, current_position.1);

    if current_symbol == "S" {
        let above_symbol = map[above_position.0][above_position.1];
        let below_symbol = map[below_position.0][below_position.1];
        let left_symbol = map[left_position.0][left_position.1];
        let right_symbol = map[right_position.0][right_position.1];
        if above_symbol == "F" || above_symbol == "|" || above_symbol == "7" {
            return above_position;
        } else if below_symbol == "J" || below_symbol == "|" || below_symbol == "L" {
            return below_position;
        } else if left_symbol == "F" || left_symbol == "-" || left_symbol == "L" {
            return left_position;
        } else if right_symbol == "J" || right_symbol == "-" || right_symbol == "7" {
            return right_position;
        } else {
            panic!("No next symbol found!");
        }
    }

    if current_symbol == "F" {
        if previous_position == below_position {
            return right_position;
        } else {
            return below_position;
        }
    }

    if current_symbol == "J" {
        if previous_position == above_position {
            return left_position;
        } else {
            return above_position;
        }
    }

    if current_symbol == "|" {
        if previous_position == above_position {
            return below_position;
        } else {
            return above_position;
        }
    }

    if current_symbol == "-" {
        if previous_position == left_position {
            return right_position;
        } else {
            return left_position;
        }
    }

    if current_symbol == "7" {
        if previous_position == left_position {
            return below_position;
        } else {
            return left_position;
        }
    }

    if current_symbol == "L" {
        if previous_position == above_position {
            return right_position;
        } else {
            return above_position;
        }
    }

    panic!("No next symbol found!");
}
fn main() {
    let file_path: String = env::args().nth(1).expect("Please supply a file path");
    let file_content: String = fs::read_to_string(file_path).unwrap();
    let lines = file_content.lines();

    let mut map: Vec<Vec<&str>> = Vec::new();
    let mut start: (usize, usize) = (0, 0);

    for (index, line) in lines.enumerate() {
        let mut symbols: Vec<&str> = line.split("").filter(|x| x.len() > 0).collect();
        let start_index = symbols.iter().position(|&x| x == "S");
        if start_index.is_some() {
            start = (start_index.unwrap() + 1, index + 1);
        }
        symbols.insert(0, ".");
        symbols.push(".");
        map.push(symbols);
    }

    let padding = vec!["."; map[0].len()];
    map.insert(0, padding.clone());
    map.push(padding.clone());

    println!("{:?}", start);

    let mut current_position = start;
    let mut previous_position = start;
    let mut steps = 0;

    loop {
        let current_symbol = map[current_position.1][current_position.0];
        if current_symbol == "S" && steps > 0 {
            println!("Found exit after {} steps", steps);
            println!("{:?}", steps / 2);
            break;
        }
        println!("{:?}", current_symbol);
        let next_position =
            find_next_position(&map, current_symbol, current_position, previous_position);
        previous_position = current_position;
        current_position = next_position;

        steps += 1;
    }
}
