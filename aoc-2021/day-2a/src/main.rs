use std::fs;

fn main() {

    let file = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");
    
    // coordinates
    let mut x = 0;
    let mut y = 0;

    for line in file.lines() {
        let thing : Vec<&str> = line.split_whitespace().collect();
        let action : &str = thing[0];
        let value : i32 = thing[1].parse::<i32>().unwrap();
        match action {
            "forward" => {x += value},
            "up" => {y -= value},
            "down" => {y += value},
            _ => println!("actiont {} does not equal any value", action),
        }

        println!("we go {} {} to position ({}, {})", value, action, x, y);
    }
    println!("result is {}", x * y);

}
