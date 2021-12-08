use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    const GRID_SIZE: usize = 1000;
    let mut hot_spots = 0;

    let mut grid = vec![vec![0; GRID_SIZE]; GRID_SIZE];

    for line in input.lines() {
        let coordinates: Vec<&str> = line.split(" -> ").collect();
        let from: Vec<&str> = coordinates[0].split(",").collect();
        let to: Vec<&str> = coordinates[1].split(",").collect();
        let from_x: i32 = from[0].parse::<i32>().unwrap();
        let from_y: i32 = from[1].parse::<i32>().unwrap();
        let to_x: i32 = to[0].parse::<i32>().unwrap();
        let to_y: i32 = to[1].parse::<i32>().unwrap();


        let dx = to_x - from_x;
        let dy = to_y - from_y;
        let mut x_dir = 0;
        let mut y_dir = 0;
        if dx != 0 {
            x_dir = dx / dx.abs();
        }
        if dy != 0 {
            y_dir = dy / dy.abs();
        }
        let length = dy.abs().max(dx.abs()) + 1;

        for i in 0..length {
            let x = from_x + i * x_dir;
            let y = from_y + i * y_dir;

            grid[x as usize][y as usize] += 1;
            if grid[x as usize][y as usize] == 2 {
                hot_spots += 1;
            }

        }
    }
    println!("{}", hot_spots);
}