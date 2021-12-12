use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let mut count = 0;
    
    for line in file.lines() {
        let mut stack = vec![];
        for character in line.split("") {
            match character {
                "<" | "(" | "{" | "[" => stack.push(character),
                ">" => if stack.pop().unwrap() != "<" {count += 25137; break;},
                "}" => if stack.pop().unwrap() != "{" {count += 1197; break;},
                "]" => if stack.pop().unwrap() != "[" {count += 57; break;},
                ")" => if stack.pop().unwrap() != "(" {count += 3; break;},
                _ => {}
            }
        }
    }
    println!("{}", count);
}