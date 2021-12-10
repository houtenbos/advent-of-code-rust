use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let mut count = 0;
    let check = vec![2, 3, 4, 7];
    for line in input.lines() {
        let entry: Vec<&str> = line.split(" | ").collect();
        let output: Vec<&str> = entry[1].split_whitespace().collect();
        for digit in output {
            if check.contains(&digit.len()) {
                count = count + 1;
            };
        };
    };
    println!("{}", count);
}