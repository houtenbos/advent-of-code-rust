use std::fs;
use std::collections::VecDeque;
use std::collections::HashSet;

const WINDOW: usize = 14;

fn main(){
    let file = fs::read_to_string("input.txt").unwrap();
    let mut buffer: VecDeque<char> = VecDeque::new();
    let mut set: HashSet<char> = HashSet::new();
    
    let mut count = 0;
    for c in file.chars() {
        set.insert(c);
        buffer.push_front(c);
        if buffer.len() > WINDOW {
            let old_c = buffer.pop_back().unwrap();
            if !buffer.contains(&old_c) {
                set.remove(&old_c);
            }
        }
        if set.len() == WINDOW {
            println!("{}", count);
            break;
        }
        count += 1;
    }
}
