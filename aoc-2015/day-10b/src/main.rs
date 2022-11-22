use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let mut scores = vec![];
    
    for line in file.lines() {
        let mut stack = vec![];
        let mut is_corrupted = false;

        for character in line.split("") {
            match character {
                "<" | "(" | "{" | "[" => stack.push(character),
                ">" => if stack.pop().unwrap() != "<" {is_corrupted = true},
                "}" => if stack.pop().unwrap() != "{" {is_corrupted = true},
                "]" => if stack.pop().unwrap() != "[" {is_corrupted = true},
                ")" => if stack.pop().unwrap() != "(" {is_corrupted = true},
                _ => {}
            }
        }
        if !is_corrupted {
            scores.push(calculate_score(&stack));
        }
    }
    scores.sort();
    println!("{}", scores[(scores.len() / 2 )]);

}

fn calculate_score(line: &Vec<&str> ) -> i64 {
    let mut score = 0;
    for character in line.iter().rev() {
        score *= 5;
        match character {
            &"<" => {score += 4}
            &"{" => {score += 3}
            &"[" => {score += 2}
            &"(" => {score += 1}
            _ => {}
        }
    }
    score
}