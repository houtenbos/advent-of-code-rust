use std::fs;
use std::collections::HashMap;

fn main(){
    let file = fs::read_to_string("index.txt").unwrap();
}

fn bring_present(){
}

fn move_santa(action: char, location: (i32, i32)) {
    let new_location = location;

    match action {
        '<' => new_location[0] -= 1,
        '>' => new_location[0] += 1,
        '^' => new_location[1] += 1,
        'v' => new_location[1] -= 1,
         _ => {},
    }
}
