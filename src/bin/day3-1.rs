use core::str::Lines;
use regex::Regex;
use std::cmp::min;
use std::env;
use std::fs;

struct Number {
    start: i32,
    end: i32,
    line: i32,
    value: i32,
}

struct Symbol {
    position: i32,
    line: i32,
    _value: char,
}

fn main() {
    let file_path: String = env::args().nth(1).expect("Please supply a file path");
    let file_content: String = fs::read_to_string(file_path).unwrap();
    let lines: Lines = file_content.lines();

    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    let number_re = Regex::new(r"[0-9]+").unwrap();
    let symbol_re = Regex::new(r"[^0-9\.]").unwrap();

    let mut valid_total = 0;

    for (index, line) in lines.enumerate() {
        for pos in number_re.find_iter(line) {
            let start = pos.start();
            let end = pos.end() - 1;
            let value = line[start..end + 1].parse::<i32>().unwrap();
            let result = Number {
                start: start as i32,
                end: end as i32,
                line: index as i32,
                value: value,
            };
            numbers.push(result);
        }
        for pos in symbol_re.find_iter(line) {
            let start = pos.start();
            let end = pos.end();
            let value = line[start..end].parse::<char>().unwrap();
            let result = Symbol {
                position: start as i32,
                line: index as i32,
                _value: value,
            };
            symbols.push(result);
        }
    }

    for number in numbers {
        let has_close_symbol = symbols.iter().any(|symbol| {
            let line_diff = (symbol.line - number.line).abs();
            let start_diff = (symbol.position - number.start).abs();
            let end_diff = (symbol.position - number.end).abs();
            let pos_diff = min(start_diff, end_diff);
            let is_close = line_diff <= 1 && pos_diff <= 1;
            return is_close;
        });

        if has_close_symbol {
            valid_total += number.value;
        }
    }

    println!("Total: {}", valid_total);
}
