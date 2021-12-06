use std::fs;

const WIDTH: usize = 12;
const COUNT: usize = 1000;

fn main() {
    let file = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let mut n_lines = 0;
    let mut bit_counter = [0; WIDTH];
    for line in file.lines() {
        let bits: Vec<char> = line.chars().collect();
        let mut i = 0;
        for bit in bits.iter() {
            bit_counter[i] += bit.to_digit(10).unwrap();
            i = i + 1;
        }
        n_lines += 1;
    }
    let mut gamma = 0;
    let mut epsilon = 0;
    let half = (COUNT / 2) as u32;
    let base: i32 = 2;

    for i in (0..WIDTH).rev() {
        println!("bit {} has {} of {}", i, bit_counter[i], n_lines);
        if bit_counter[i] > half {
            gamma += base.pow((WIDTH-i-1) as u32);
        }
        else {
            epsilon += base.pow((WIDTH-i-1) as u32);
        }
        println!("gamma {} epsilon {}", gamma, epsilon);
    }
    println!("result: {}", gamma * epsilon);
}
