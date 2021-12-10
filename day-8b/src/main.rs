use std::fs::read_to_string;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let mut total_value = 0;
    for line in input.lines() {
        let entry: Vec<&str> = line.split(" | ").collect();
        let signal: Vec<&str> = entry[0].split_whitespace().collect();
        let output: Vec<&str> = entry[1].split_whitespace().collect();
        
        let map = decode(signal);
        let mut value: i32 = 0;
        for i in 0..4i32 {
            let n = check_map(output[i as usize], &map);
            let pow: u32 = 3-i as u32;
            value += n * i32::pow(10, pow);
        }
        println!("{}", value);
        total_value += value;

    }
    println!("{}", total_value);
}
fn check_map(code: &str, map: &HashMap<i32, HashSet<char>>) -> i32 {
    let code_set: HashSet<char> = code.chars().collect();
    let mut n = 0;
    for (number, number_code) in map {
        if code_set.symmetric_difference(&number_code).cloned().collect::<HashSet<char>>().len() == 0 {
            n = *number;
            break;
        };
    };
    return n;
}

fn decode(signal: Vec<&str>) -> HashMap<i32, HashSet<char>> {
    let mut mapping: HashMap<i32, HashSet<char>> = HashMap::new();
    for _i in 0..10 {
        for code in &signal {
            let code_set: HashSet<char> = code.chars().collect();
            
            if code.len() == 2 {
                mapping.insert(1, code_set);
            }
            else if code.len() == 3 {
                mapping.insert(7, code_set);
            }
            else if code.len() == 4 {
                mapping.insert(4, code_set);
            }
            else if code.len() == 5 {
                if mapping.contains_key(&1) && mapping.contains_key(&4) {
                    let one: HashSet<char> = mapping.get(&1).unwrap().clone();
                    let four: HashSet<char> = mapping.get(&4).unwrap().clone();
                    let diff1 = one.symmetric_difference(&code_set).cloned().collect::<HashSet<char>>().len();
                    let diff4 = four.symmetric_difference(&code_set).cloned().collect::<HashSet<char>>().len();
                    let diff5 = diff4 + diff1;
                    match diff5 {
                        10 => mapping.insert(2, code_set),
                        6 => mapping.insert(3, code_set),
                        8 => mapping.insert(5, code_set),
                        _ => {
                            println!("1: {:?}", mapping.get(&1).unwrap());
                            println!("4: {:?}", mapping.get(&4).unwrap());
                            println!("Diff is {}, set {:?} vs {:?} and {:?}", diff5, code_set, one, four);
                            mapping.insert(99, code_set)
                        },
                    };
                };
            }
            else if code.len() == 6 {
                if mapping.contains_key(&7) && mapping.contains_key(&4) && mapping.contains_key(&1) {
                    let one: HashSet<char> = mapping.get(&1).unwrap().clone();
                    let combo: HashSet<char> = mapping.get(&7).unwrap().union(&mapping.get(&4).unwrap()).cloned().collect();
                    let diff = combo.symmetric_difference(&code_set).cloned().collect::<HashSet<char>>().len() + 
                                     one.symmetric_difference(&code_set).cloned().collect::<HashSet<char>>().len();
                    match diff {
                        5 => mapping.insert(9, code_set),
                        9 => mapping.insert(6, code_set),
                        7 => mapping.insert(0, code_set),
                        _ => {
                            println!("4: {:?}", mapping.get(&4).unwrap());
                            println!("7: {:?}", mapping.get(&7).unwrap());
                            println!("Diff is {}, set {:?} vs {:?}", diff, code_set, combo);
                            mapping.insert(99, code_set)
                        },
                    };
                };
            }
            else if code.len() == 7 {
                mapping.insert(8, code_set);
            };
        };
    }
    return mapping;
}