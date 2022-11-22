use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    const GRID_SIZE: usize = 1000;
    let mut hot_spots = 0;

    // grid
    let mut grid = vec![vec![0; GRID_SIZE]; GRID_SIZE];

    for line in input.lines() {
        let coordinates: Vec<&str> = line.split(" -> ").collect();
        let from: Vec<&str> = coordinates[0].split(",").collect();
        let to: Vec<&str> = coordinates[1].split(",").collect();
        let from_x: i32 = from[0].parse::<i32>().unwrap();
        let from_y: i32 = from[1].parse::<i32>().unwrap();
        let to_x: i32 = to[0].parse::<i32>().unwrap();
        let to_y: i32 = to[1].parse::<i32>().unwrap();

        let dx = (from_x - to_x).abs();
        let dy = (from_y - to_y).abs();

        if dy == 0 {
            let start_x: i32 = *vec![from_x, to_x].iter().min().unwrap();
            let end_x = start_x + dx + 1;
            let y= from_y;
            for x in start_x..end_x {
                grid[x as usize][y as usize] += 1;
                if grid[x as usize][y as usize] == 2 {
                    hot_spots += 1;
                }
            }
        }
        else if dx == 0 {
            let start_y: i32 = *vec![from_y, to_y].iter().min().unwrap();
            let end_y = start_y + dy + 1;
            let x = from_x;
            for y in start_y..end_y {
                grid[x as usize][y as usize] += 1;
                if grid[x as usize][y as usize] == 2 {
                    hot_spots += 1;
                }
            }
        }
    }
    println!("{}", hot_spots);
}
