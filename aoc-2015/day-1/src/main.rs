use std::fs;

fn main() {
    let file = fs::read_to_string("./input copy.txt").expect("Something went wrong reading the file");
    let mut count: i32 = 0;
    let mut first_basement = 0;
    let mut i = 0;

    for character in file.split("") {
        match character {
            "(" => count += 1,
            ")" => count -= 1,
            _ => {continue}
        };
        i+=1;
        if first_basement == 0 && count == -1{
            first_basement = i;
        }
    };

    println!("Final floor: {}", count);
    println!("First time in basement: {}", first_basement);
}


// read file fs::read_to_string().unwrap();
// loop over string for character in string {}
// match => match var {case => {}, case => {}, _ => {}}



