use num::integer::lcm;
use std::collections::HashMap;
use std::env;
use std::fs;
fn main() {
    let file_path: String = env::args().nth(1).expect("Please supply a file path");
    let file_content: String = fs::read_to_string(file_path).unwrap();
    let mut lines = file_content.lines();

    let first_line = lines.next().unwrap();
    let moves = first_line
        .split("")
        .filter(|x| x.len() > 0)
        .collect::<Vec<&str>>();

    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut current_nodes: Vec<&str> = Vec::new();

    for line in lines {
        if line.len() == 0 {
            continue;
        }
        let (key, value) = line[0..line.len() - 1].split_once(" = (").unwrap();
        let (left, right) = value.split_once(", ").unwrap();
        println!("{:?} {:?} {:?}", key, left, right);
        nodes.insert(key, (left, right));
        if key.chars().nth(2) == Some('A') {
            current_nodes.push(key);
        }
    }

    let mut loop_sizes: Vec<i64> = Vec::new();

    for idx in 0..current_nodes.len() {
        let mut current_node = current_nodes[idx];
        let mut iterations: i64 = 0;
        loop {
            let remainder = iterations % moves.len() as i64;
            if current_node.chars().nth(2) == Some('Z') && remainder == 0 {
                println!("Iterations: {:?}", iterations);
                loop_sizes.push(iterations);
                break;
            }
            let (left, right) = nodes.get(current_node).unwrap();
            let current_move = moves[remainder as usize];
            current_node = match current_move {
                "L" => left,
                "R" => right,
                _ => panic!("Unknown move"),
            };
            iterations += 1;
        }
    }

    println!("Iterations: {:?}", loop_sizes);
    let lowest_common_multiple = loop_sizes.iter().fold(1, |acc, x: &i64| lcm(acc, *x));
    println!("Lowest common multiple: {:?}", lowest_common_multiple);
}
