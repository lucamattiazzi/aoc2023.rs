use std::env;
use std::fs;

fn parse_line(line: &str) -> f64 {
    let values_string = line.split_once(":").unwrap().1;

    let values: Vec<&str> = values_string.split_whitespace().collect();
    let string_value = values.join("");
    let value: f64 = string_value.parse::<f64>().unwrap();
    return value;
}

fn main() {
    let file_path: String = env::args().nth(1).expect("Please supply a file path");
    let file_content: String = fs::read_to_string(file_path).unwrap();
    let mut lines = file_content.lines();
    let time_line = lines.next().unwrap();
    let distance_line = lines.next().unwrap();
    let time: f64 = parse_line(time_line);
    let distance: f64 = parse_line(distance_line);

    let x1 = (time + (time.powi(2) - distance * 4.0).sqrt()) / 2.0;
    let x2 = (time - (time.powi(2) - distance * 4.0).sqrt()) / 2.0;

    let higher = f64::max(x1, x2);
    let lower = f64::min(x1, x2);

    let diff = higher.ceil() - lower.floor() - 1.0;

    println!("{}", diff as i64)
}
