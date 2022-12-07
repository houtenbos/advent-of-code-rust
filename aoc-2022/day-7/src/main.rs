use std::fs;
use std::collections::HashMap;

const DISK_SPACE: i32 = 70000000;
const MIN_FREE_SPACE: i32 = 30000000;

fn main() {
    let file = fs::read_to_string("index.txt").unwrap();
    let mut directories: HashMap<String, i32> = HashMap::new();
    let mut current_dir: Vec<String> = vec![];
    for line in file.lines(){
        match &line[0..4] {
            "$ cd" => cd(&mut current_dir, line[5..].to_owned()),
            "$ ls" => {},
            "dir " => {},
            _ => {
                let file_size = line.split(" ").collect::<Vec<&str>>()[0].parse::<i32>().unwrap();
                add_file(&mut directories, &mut current_dir, file_size);
            }

        }
    }

    let total_used = directories.get(&"/".to_owned()).unwrap();
    let free_space = DISK_SPACE - total_used;
    let to_delete = MIN_FREE_SPACE - free_space;
    let mut gonna_delete = DISK_SPACE;

    for (k,val) in directories.iter(){
        println!("{}, {}", k ,val);
        if val > &to_delete {
            if val < &gonna_delete {
                gonna_delete = val.to_owned();
            }
            
        }
    }
    println!("gonna delete {}", gonna_delete);
}

fn cd(current_dir: &mut Vec<String>, dir: String ){
    if dir == ".." {
        current_dir.pop();
    }else{
        current_dir.push(dir);
    }
}

fn add_file(dirs: &mut HashMap<String, i32>, current_dir: &mut Vec<String>, size: i32 ){
    for i in 0..current_dir.len(){
        let dir = current_dir[0..i+1].join("/").to_owned();
        // check if dir is there and add to 
        match dirs.get(&dir).cloned() {
            Some(v) => dirs.insert(dir, v + size),
            None => dirs.insert(dir, size)
        };
    }
}
