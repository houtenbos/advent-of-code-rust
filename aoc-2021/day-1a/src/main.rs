use std::fs;

fn main() {

    let file = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");
    
    let mut increases = 0;
    let mut prev = i32::MAX;
    for line in file.lines() {
        let value = line.parse::<i32>().unwrap();
        if value > prev {
            increases += 1;
        }
        prev = value;
    }

    println!("Submarine depth increased {} times", increases);

}
