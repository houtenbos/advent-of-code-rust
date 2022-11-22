use std::collections::HashMap;

fn main() {
    let (temp, rules) = include_str!("../input.txt").trim().split_once("\n\n").unwrap();
    let mut rule_map: HashMap<&str, &str> = HashMap::new();
    for line in rules.lines(){
        let (code, result) = line.split_once(" -> ").unwrap();
        rule_map.insert(code, result);
    }
    let mut template = temp.clone().to_owned();
    for _i in 0..40{
        let mut new_template = String::new();
        for j in 0..(template.len() - 1){
            let slice = &template[j..j+2];
            let insert = rule_map.get(&slice).unwrap().clone();
            if new_template.len() == 0 {
                new_template.push(slice.chars().nth(0).unwrap());
            }
            new_template.push_str(insert);
            new_template.push(slice.chars().nth(1).unwrap());
        }
        template = new_template;
        println!("{}", template.len());
    }

    let mut char_count: HashMap<char, i32> = HashMap::new();
    for c in template.chars() {
        if char_count.contains_key(&c) {
            *char_count.get_mut(&c).unwrap() += 1;
        }
        else {
            char_count.insert(c, 1);
        }
    }

    let mut max = 0;
    let mut min = i32::MAX;
    for v in char_count.values(){
        if v > &max {
            max = v.clone();
        }
        if v < &min {
            min = v.clone();
        }
    }
    println!("{}", max-min);
}