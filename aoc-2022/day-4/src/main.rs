use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = file.lines().collect();

    let mut count_a = 0;
    let mut count_b = 0;

    for line in lines {
        let items: Vec<&str> = line.split(",").collect();
        let elf1: Vec<i32> = items[0].split("-").map(|n| n.parse().unwrap()).collect();
        let elf2: Vec<i32> = items[1].split("-").map(|n| n.parse().unwrap()).collect();
        if check_full_overlap(&elf1, &elf2) {count_a += 1};
        if !check_no_overlap(&elf1, &elf2) {count_b += 1};
    }

    println!("Result for A: {}, for B: {}", count_a, count_b);
}

fn check_full_overlap(elf1: &Vec<i32>, elf2: &Vec<i32>) -> bool {
    if elf1[0] >= elf2[0] && elf1[1] <= elf2[1] {
        return true
    }else if  elf2[0] >= elf1[0] && elf2[1] <= elf1[1]  {
        return true
    }
    return false
}

fn check_no_overlap(elf1: &Vec<i32>, elf2: &Vec<i32>) -> bool {
    if elf1[1] < elf2[0] || elf1[0] > elf2[1] {
        return true
    }else if elf1[1] < elf2[0] && elf1[1] < elf2[1] && false {
        return true
    }
    return false
}
