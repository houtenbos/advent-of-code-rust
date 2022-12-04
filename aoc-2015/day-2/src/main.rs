use std::fs;
use std::cmp::min;
use std::cmp::max;

fn main() {

    let file = fs::read_to_string("./input.txt").expect("Something went wrong reading the file");

    let mut paper = 0;
    let mut ribbon = 0;

    for line in file.lines() {
        let vec: Vec<i32> = line.split("x").map(|x| x.parse::<i32>().unwrap()).collect();

        let (l, b, h): (i32, i32, i32) = (vec[0], vec[1], vec[2]);

        paper += 2 * l * b + 2 * l * h + 2 * b * h + min(min(l * b, l * h), b * h);
        ribbon += l * b * h + 2 * (l + b + h - max(max(l, b), h));
    }
    println!("paper: {}, ribbon: {}", paper, ribbon);
}
