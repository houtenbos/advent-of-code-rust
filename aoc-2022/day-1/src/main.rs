use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let mut count = 0;
    let mut calories: Vec<i32> = Vec::new();
    for line in file.lines() {
        match line.parse::<i32>() {
            Ok(cal) => {count += cal},
            Err(_) => {calories.push(count); count = 0}
        }
    }

    // a: get max value
    let top = calories.iter().max().unwrap();
    println!("top: {}", top );

    // b: get top 3 values
    calories.sort_by(|a, b| b.cmp(a));
    let top_3 = calories[0] + calories[1] + calories[2];
    println!("top 3: {}", top_3 );
}

