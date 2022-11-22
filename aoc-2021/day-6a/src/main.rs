use std::fs::read_to_string;

fn main() {
    const DAYS: i32 = 80;
    let mut fishes: Vec<i32> = read_to_string("input.txt")
                                .unwrap()
                                .split(",")
                                .map(|n| n.parse().unwrap())
                                .collect();
    for _day in 0..DAYS {
        let n_fishes = fishes.len();
        for i in 0..n_fishes {
            if fishes[i] == 0 {
                fishes[i] = 6;
                fishes.push(8);
            }
            else{
                fishes[i] -= 1;
            }
        }
        println!("After {} days: {:?}", _day + 1, fishes.len());
    }
    println!("After {} days: {:?}", DAYS, fishes.len());
}
