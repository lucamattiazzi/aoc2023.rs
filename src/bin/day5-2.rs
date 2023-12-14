use std::env;
use std::fs;

fn convert_in_range(ranges: Vec<Vec<i64>>, number: i64) -> i64 {
    for range in ranges {
        let from_start = range[1];
        let from_end = range[1] + range[2];
        let to_start = range[0];
        if number >= from_start && number < from_end {
            return number + to_start - from_start;
        }
    }
    return number;
}

fn converter(all_ranges: Vec<Vec<Vec<i64>>>, starting_number: i64) -> i64 {
    let mut number = starting_number;
    for ranges in all_ranges {
        number = convert_in_range(ranges, number);
    }
    return number;
}

fn main() {
    let file_path: String = env::args().nth(1).expect("Please supply a file path");
    let file_content: String = fs::read_to_string(file_path).unwrap();
    let mut seeds: Vec<i64> = Vec::new();
    let mut all_ranges: Vec<Vec<Vec<i64>>> = Vec::new();
    for block in file_content.split("\n\n") {
        let lines = block.lines().collect::<Vec<&str>>();
        if lines.len() == 1 {
            let block_seeds: Vec<i64> = lines[0]
                .split(": ")
                .nth(1)
                .unwrap()
                .split(" ")
                .map(|x| x.parse::<i64>().unwrap())
                .collect();
            block_seeds.chunks(2).for_each(|seed| {
                for i in seed[0]..seed[0] + seed[1] {
                    seeds.push(i);
                }
            });
        } else {
            let mut ranges: Vec<Vec<i64>> = Vec::new();
            for (index, line) in lines.iter().enumerate() {
                if index > 0 {
                    let range: Vec<i64> =
                        line.split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
                    ranges.push(range);
                }
            }
            all_ranges.push(ranges);
        }
    }
    println!("{:?}", seeds.len());
    let converted_numbers = seeds
        .iter()
        .map(|x| converter(all_ranges.clone(), *x))
        .collect::<Vec<i64>>();
    let min = *converted_numbers.iter().min().unwrap();
    println!("min: {:?}", min);
}
