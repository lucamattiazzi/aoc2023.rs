use std::env;
use std::fs;

fn parse_line(line: &str) -> Vec<f32> {
    let values_string = line.split_once(":").unwrap().1;

    let values: Vec<f32> = values_string
        .split_whitespace()
        .map(|x| x.parse::<f32>().unwrap())
        .collect();
    return values;
}

fn main() {
    let file_path: String = env::args().nth(1).expect("Please supply a file path");
    let file_content: String = fs::read_to_string(file_path).unwrap();
    let mut lines = file_content.lines();
    let time_line = lines.next().unwrap();
    let distance_line = lines.next().unwrap();
    let times: Vec<f32> = parse_line(time_line);
    let distances: Vec<f32> = parse_line(distance_line);

    let mut total: i32 = 1;

    for (time, distance) in times.iter().zip(distances.iter()) {
        let x1 = (time + (time.powi(2) - distance * 4.0).sqrt()) / 2.0;
        let x2 = (time - (time.powi(2) - distance * 4.0).sqrt()) / 2.0;

        let higher = f32::max(x1, x2);
        let lower = f32::min(x1, x2);

        let diff = higher.ceil() - lower.floor() - 1.0;
        total *= diff as i32;
    }

    println!("{}", total)
}
