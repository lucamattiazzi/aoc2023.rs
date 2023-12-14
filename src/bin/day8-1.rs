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
    let mut current_node: &str = "AAA";

    for line in lines {
        if line.len() == 0 {
            continue;
        }
        let (key, value) = line[0..line.len() - 1].split_once(" = (").unwrap();
        let (left, right) = value.split_once(", ").unwrap();
        println!("{:?} {:?} {:?}", key, left, right);
        nodes.insert(key, (left, right));
    }

    let mut iterations = 0;

    loop {
        if current_node == "ZZZ" {
            println!("Iterations: {:?}", iterations);
            break;
        }
        let (left, right) = nodes.get(current_node).unwrap();
        let current_move = moves[iterations % moves.len()];
        println!(
            "Current node: {:?} {:?} {:?} {:?}",
            current_node, left, right, current_move
        );
        current_node = match current_move {
            "L" => left,
            "R" => right,
            _ => panic!("Unknown move"),
        };
        iterations += 1;
    }
}
