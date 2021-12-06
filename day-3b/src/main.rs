use std::fs;

const WIDTH: usize = 12;
const COUNT: usize = 1000;

fn main() {
    let file = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let mut n_lines = 0;
    let mut bit_counter = [0; WIDTH];
    let mut bit_collection = [[0; WIDTH]; COUNT];

    for line in file.lines() {
        let bits: Vec<char> = line.chars().collect();
        let mut i = 0;
        for bit in bits.iter() {
            bit_counter[i] += bit.to_digit(10).unwrap();
            bit_collection[n_lines][i] = bit.to_digit(10).unwrap();
            i = i + 1;
        }
        n_lines += 1;
    }
    let half = (COUNT / 2) as u32;
    let base: i32 = 2;

    let mut skip_bits = Vec::new();
    for j in 0..WIDTH {
        let mut count = 0;
        for i in 0..COUNT {
            if skip_bits.contains(&i) {
                continue;
            }
            if bit_collection[i][j] == 1 {
                count += 1;
            }
        }
        let mut n_skip = 1;
        if count*2 >= (COUNT - skip_bits.len()) {
            n_skip = 0;
        }
        println!("Count is {} and n lines is {}, skip is {}", count, (COUNT - skip_bits.len()), n_skip);
        for i in 0..COUNT {
            if skip_bits.contains(&i) {
                continue;
            }
            if bit_collection[i][j] == n_skip {
                skip_bits.push(i);
            }
        }
    }
    let mut co2_skip_bits = Vec::new();
    for j in 0..WIDTH {
        let mut count = 0;
        for i in 0..COUNT {
            if co2_skip_bits.contains(&i) {
                continue;
            }
            if bit_collection[i][j] == 1 {
                count += 1;
            }
        }
        let mut n_skip = 0;
        if count*2 >= (COUNT - co2_skip_bits.len()) {
            n_skip = 1;
        }
        println!("Count is {} and n lines is {}, skip is {}", count, (COUNT - co2_skip_bits.len()), n_skip);
        if( COUNT - co2_skip_bits.len() == 1 ){
            continue;
        }
        for i in 0..COUNT {
            if co2_skip_bits.contains(&i) {
                continue;
            }
            if bit_collection[i][j] == n_skip {
                co2_skip_bits.push(i);
            }
        }
    }
    let mut co2 = 0;
    let mut oxy = 0;
    for i in 0..COUNT {
        if skip_bits.contains(&i) {
            continue;
        }
        else {
            oxy = bits_to_decimal(&bit_collection[i]);
        }
    }

    let mut co2_bits = [0; WIDTH];
    for i in 0..COUNT {
        if co2_skip_bits.contains(&i) {
            continue;
        }
        else {
            co2 = bits_to_decimal(&bit_collection[i]);
        }
    }
    println!("Oxygen {}, CO2 {}, answer {}", oxy, co2, oxy * co2);
}

fn bits_to_decimal(bits: &[u32]) -> i32{
    let base: i32 = 2;
    let mut dec: i32 = 0;
    for i in (0..WIDTH).rev() {
        if( bits[i] == 1){
            dec += base.pow((WIDTH-i-1) as u32);
        }
    }
    return dec;
}
