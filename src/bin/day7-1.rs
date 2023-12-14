use counter::Counter;
use std::env;
use std::fs;

const VALUES: &[&str] = &[
    "2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A",
];

fn count_hand(hand: &str) -> Counter<&str> {
    let count: Counter<&str> = hand
        .split("")
        .filter(|x| x.len() > 0)
        .collect::<Counter<&str>>();
    return count;
}

fn determine_hand_points(hand: &Counter<&str>) -> i64 {
    let hand_ordered: Vec<(&str, usize)> = hand.most_common_ordered();
    let highest = hand_ordered[0].1;
    if highest == 5 {
        return 6;
    } else if highest == 4 {
        return 5;
    } else if highest == 3 {
        if hand_ordered[1].1 == 2 {
            return 4;
        } else {
            return 3;
        }
    } else if highest == 2 {
        if hand_ordered[1].1 == 2 {
            return 2;
        } else {
            return 1;
        }
    }
    return 0;
}

fn determine_remaining_hand_points(hand: &str) -> Vec<i64> {
    let mut points: Vec<i64> = Vec::new();
    let letters = hand.split("").collect::<Vec<&str>>();
    for letter in letters {
        let index = VALUES.iter().position(|&r| r == letter);
        if !index.is_none() {
            points.push(index.unwrap() as i64);
        }
    }
    return points;
}

fn main() {
    let file_path: String = env::args().nth(1).expect("Please supply a file path");
    let file_content: String = fs::read_to_string(file_path).unwrap();
    let lines = file_content.lines();

    let mut all_hands: Vec<(&str, Vec<i64>, i64)> = Vec::new();
    let mut total_points: i64 = 0;

    for line in lines {
        let (hand, points) = line.split_once(" ").unwrap();
        let hand_count = count_hand(hand);
        let int_hand_rank: i64 = determine_hand_points(&hand_count);
        let mut remaining_points = determine_remaining_hand_points(&hand);
        remaining_points.insert(0, int_hand_rank);
        all_hands.push((hand, remaining_points, points.parse::<i64>().unwrap()));
    }

    all_hands.sort_unstable_by_key(|hand| hand.1.clone());

    for (index, hand) in all_hands.iter().enumerate() {
        println!(
            "{:?} : {:?} - {} - {}",
            hand.0,
            hand.1,
            hand.2,
            (index as i64 + 1)
        );
        total_points += hand.2 * (index as i64 + 1);
    }
    println!("{:?}", total_points);
}
