use std::fs;
const NEXT: [(isize, isize); 8] = [(0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1)];

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut dumbo_map = vec![vec![0; 10]; 10];
    let mut i = 0;
    for line in input.lines() {
        dumbo_map[i] = line.chars().map(|n| n.to_digit(10).unwrap()).collect::<Vec<u32>>();
        i += 1;
    };

    let mut n_flashes = 0;
    for _i in 0..100 {
        // add 1 to all octopusses
        for j in 0..10 {
            for k in 0..10 {
                dumbo_map[j][k] += 1;
            }
        }
        // flash
        for j in 0..10 {
            for k in 0..10 {
                n_flashes += flash(&mut dumbo_map, j, k);
            }
        }
    }
    println!("{}", n_flashes);
}

fn flash(dumbo_map: & mut Vec<Vec<u32>>, x: usize, y: usize) -> i32 {
    if  dumbo_map[x][y] <= 9  {
        return 0;
    }
    let mut counter = 1;
    dumbo_map[x][y] = 0;

    let adjecent_dumbos: Vec<(usize, usize)> = NEXT.iter().map(|(xx, yy)| ((x as isize + xx) as usize, (y as isize + yy) as usize)).collect();
    for (xx, yy) in adjecent_dumbos{
        if xx < 10 && yy < 10 {
            if dumbo_map[xx][yy] != 0 {
                dumbo_map[xx][yy] += 1;
            }
            if dumbo_map[xx][yy] >= 9 {
                counter += flash( dumbo_map, xx, yy);
            }
        }
    }
    counter
}



