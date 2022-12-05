use std::fs;
use std::collections::VecDeque;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let file_stack = fs::read_to_string("input-stack.txt").unwrap();

    let mut container_stack: Vec<VecDeque<&str>> = vec![
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
    ];

    // parse stack
    for line in file_stack.lines() {
        for i in 0..36 {
            if i % 4 == 0 {
                let ind = i / 4;
                let container = &line[i+1..i+2];
                if container != ' '.to_string() {
                    container_stack[ind].push_back(&container);
                }
            }
        }
    }

    // parse move & move
    for line in file.lines() {
        let qty: usize = line.split(' ').collect::<Vec<&str>>()[1].parse::<usize>().unwrap();
        let from: usize = line.split(' ').collect::<Vec<&str>>()[3].parse::<usize>().unwrap() - 1;
        let to: usize = line.split(' ').collect::<Vec<&str>>()[5].parse::<usize>().unwrap() - 1;

        move_container(&mut container_stack, from, to, qty, 9001);
    }

    // print top containers
    print_top_containers(&container_stack);
}

fn move_container(container_stack: &mut Vec<VecDeque<&str>>, from: usize, to: usize, qty: usize, crane_type: i32) {
    let mut containers: VecDeque<&str> = VecDeque::new();

    for _ in 0..qty {
        containers.push_front(container_stack[from].pop_front().unwrap());
    }

    for _ in 0..qty {
        match crane_type {
            9000 => container_stack[to].push_front(containers.pop_back().unwrap()),
            9001 => container_stack[to].push_front(containers.pop_front().unwrap()),
            _ => panic!("Invalid crane type")
        }
    }
}

fn print_top_containers(container_stack: &Vec<VecDeque<&str>>){
    let mut top_containers = ' '.to_string();
    for i in 0..9 {
        top_containers += container_stack[i][0];
    }
    println!("top containers {}", top_containers);
}



// fn move_container_9000(container_stack: &mut Vec<VecDeque<&str>>, from: usize, to: usize, qty: usize) {
//     let container = container_stack[from].pop_front().unwrap();
//     println!("move container {} from {} to {}, qty: {}", container, from, to, qty);
//     container_stack[to].push_front(container);
//     if qty - 1  > 0 {
//         move_container_9000(container_stack, from, to, qty - 1)
//     }
// }
