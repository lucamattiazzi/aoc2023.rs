use std::env;
use std::fs;
fn main() {
    let file_path: String = env::args().nth(1).expect("Please supply a file path");
    let file_content: String = fs::read_to_string(file_path).unwrap();
    let lines = file_content.lines();

    let mut total: i64 = 0;

    for line in lines {
        let numbers: Vec<i64> = line
            .split(" ")
            .map(|i| i.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        println!("{:?}", numbers);

        let mut current_numbers = numbers.clone();
        let mut lists: Vec<Vec<i64>> = Vec::new();
        lists.push(numbers.clone());

        loop {
            let diffs = current_numbers
                .windows(2)
                .map(|w| w[1] - w[0])
                .collect::<Vec<i64>>();
            println!("{:?}", diffs);
            if diffs.iter().all(|x| x == &0) {
                break;
            }
            current_numbers = diffs.clone();
            lists.push(diffs);
        }

        lists.reverse();
        let mut first_value = 0;
        for list in lists {
            first_value = list[0] - first_value;
        }
        println!("last value: {}", first_value);
        total = total + first_value;
    }

    println!("total: {}", total);
}
