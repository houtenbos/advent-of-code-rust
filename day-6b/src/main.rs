use std::fs::read_to_string;
use std::collections::HashMap;


fn main() {
    const DAYS: i32 = 256;
    let fishes: Vec<i64> = read_to_string("input.txt")
                                .unwrap()
                                .split(",")
                                .map(|n| n.parse().unwrap())
                                .collect();

    let mut fish_age_group = HashMap::new();
    for i in 0..9 {
        fish_age_group.insert(i, 0);
    }
    for fish in &fishes {
        fish_age_group.insert(*fish, fish_age_group[&fish] + 1);
    }


    for day in 0..DAYS {
        let mut previous = fish_age_group[&8];
        fish_age_group.insert(8, 0);
        for i in (0..8).rev() {
            let this: i64 = fish_age_group[&i];
            fish_age_group.insert(i, previous);
            previous = this;
            if i == 0 {
                fish_age_group.insert(6, fish_age_group[&6] + this);
                fish_age_group.insert(8,  this);
            }
        }
        let total: i64 = fish_age_group.values().sum();
        println!("After {} days: {}", day + 1, total);
    }
    let total: i64 = fish_age_group.values().sum();
    println!("After {} days: {:?}", DAYS, total);

}
