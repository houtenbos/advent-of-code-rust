use std::fs;
use std::collections::VecDeque;

struct Coor {
    x: usize,
    y: usize
}

impl Coor {
    fn new () -> Coor {
        Coor{x: 0, y: 0}
    }
}

impl PartialEq for Coor {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Clone for Coor {
    fn clone(&self) -> Self {
        Coor {
            x: self.x,
            y: self.y
        }
    }
}

struct QueueItem {
    location: Coor,
    path: Vec<Coor>
}

fn main(){
    let file = fs::read_to_string("input.txt").unwrap();
    let mut map: Vec<Vec<i32>> = vec![];
    let mut start = Coor::new();
    let mut end = Coor::new();

    // parse input
    for (i, line) in file.lines().enumerate(){
        map.push(vec![]);
        for (j, c) in line.chars().enumerate(){
            if c == 'S' {
                start.x = i;
                start.y = j;
                map[i].push('a' as i32);
            }else if c == 'E' {
                end.x = i;
                end.y = j;
                map[i].push('z' as i32);
            }else{
                map[i].push(c as i32);
            }

        }
    }

    let path = search(&map, start, end).unwrap();

    // steps is path length minus 1
    println!("{}", path.len() - 1);

    // show_map(&map);
}

fn search(map: &Vec<Vec<i32>>, start: Coor, goal: Coor) -> Option<Vec<Coor>> {
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![1000; map[0].len()]; map.len()];
    let path = vec![];
    let mut shortest_path = vec![];

    queue.push_front(QueueItem{location: start.clone(), path});
    visited[start.x][start.y] = 0;

    while !queue.is_empty() {
        // use pop back for bfs and pop front for dfs
        let mut current = queue.pop_back().unwrap();
        current.path.push(current.location.clone());

        // check if goal is reached
        if current.location == goal {
            if shortest_path.len() == 0 || current.path.len() < shortest_path.len() {
                shortest_path = current.path.clone();
            }
        }

        // add neighboring cells to the queue if they are valid and not visited
        let neighbors: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dx, dy) in neighbors.iter() {
            let x_old = current.location.x;
            let y_old = current.location.y;
            let x_new = current.location.x as i32 + dx;
            let y_new = current.location.y as i32 + dy;

            // check for boundary
            if x_new < 0 || y_new < 0 {
                continue;
            }
            let x_new = x_new as usize;
            let y_new = y_new as usize;

            if x_new >= map.len() || y_new >= map[0].len() {
                continue;
            }

            // check for elevation difference
            if map[x_new][y_new] - map[x_old][y_old] > 1 {
                continue;
            }

            // check if we already visited the new location with less or equal steps
            if current.path.len() >= visited[x_new][y_new] {
                continue;
            }

            visited[x_new][y_new] = current.path.len();
            queue.push_front(QueueItem{location: Coor{x: x_new, y: y_new}, path: current.path.clone()});
        }
    }

    if shortest_path.len() > 0 {
        Some(shortest_path)
    }else{
        None
    }
}

