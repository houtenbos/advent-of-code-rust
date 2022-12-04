use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let mut count_a = 0;
    let mut count_b = 0;
    let lines: Vec<&str> = file.lines().collect();

    for i in 0..lines.len() {
        // a
        let line = lines[i];
        let length = line.len() / 2;
        let set1 = &line[..length];
        let set2 = &line[length..length*2];
        for c in set1.chars() {
            if set2.contains(c) {
                count_a += to_priority(c);
                break;
            }
        }
        // b
        if (i + 1)%3 == 0 {
            for c in lines[i-2].chars() {
                if lines[i-1].contains(c) && lines[i].contains(c){
                    count_b += to_priority(c);
                    break;
                }
            }
        }
    }

    println!("result: {}", count_a );
    println!("result: {}", count_b );
}

fn to_priority(c: char) -> i32{
    let ascii = c as i32;
    if ascii >= 97 { ascii - 96 } else {ascii - 64 + 26 }
}
