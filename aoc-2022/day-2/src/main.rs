use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let mut score_a = 0;
    let mut score_b = 0;
    for line in file.lines() {
        let moves: Vec<&str> = line.split(" ").collect();
        score_a += calculate_score_a(moves[0], moves[1]);
        score_b += calculate_score_b(moves[0], moves[1]);
    }
    println!("Total score a is: {}", score_a);
    println!("Total score b is: {}", score_b);
}

fn calculate_score_a(m1: &str, m2: &str) -> i32 {
    let c1 = m1.chars().collect::<Vec<char>>()[0];
    let c2 = m2.chars().collect::<Vec<char>>()[0];

    match c2 {
        'X' => {
            match c1 {
                'A' => 3 + 1,
                'B' => 0 + 1,
                'C' => 6 + 1,
                _ => panic!("WRONG INPUT: {}", m1)
            }
        },
        'Y' => {
            match c1 {
                'A' => 6 + 2,
                'B' => 3 + 2,
                'C' => 0 + 2,
                _ => panic!("WRONG INPUT: {}", m1)
            }
        },
        'Z' => {
            match c1 {
                'A' => 0 + 3,
                'B' => 6 + 3,
                'C' => 3 + 3,
                _ => panic!("WRONG INPUT: {}", m1)
            }
        },
        _ => panic!("WRONG INPUT: {}", m2)
    }
}

fn calculate_score_b(m1: &str, m2: &str) -> i32 {
    let c1 = m1.chars().collect::<Vec<char>>()[0];
    let c2 = m2.chars().collect::<Vec<char>>()[0];

    match c2 {
        'X' => {
            match c1 {
                'A' => 0 + 3,
                'B' => 0 + 1,
                'C' => 0 + 2,
                _ => panic!("WRONG INPUT: {}", m1)
            }
        },
        'Y' => {
            match c1 {
                'A' => 3 + 1,
                'B' => 3 + 2,
                'C' => 3 + 3,
                _ => panic!("WRONG INPUT: {}", m1)
            }
        },
        'Z' => {
            match c1 {
                'A' => 6 + 2,
                'B' => 6 + 3,
                'C' => 6 + 1,
                _ => panic!("WRONG INPUT: {}", m1)
            }
        },
        _ => panic!("WRONG INPUT: {}", m2)
    }
}
