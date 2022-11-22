use std::fs::read_to_string;
fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut bathymetry = vec![vec![0; 100]; 100];
    let mut i = 0;
    for line in input.lines() {
        bathymetry[i] = line.chars().map(|n| n.to_digit(10).unwrap()).collect::<Vec<u32>>();
        i += 1;
    };
    let mut count = 0;
    for y in 0..100 {
        for x in 0..100 {
            let mut heights: Vec<u32> = vec![];
            if x > 0 {
                heights.push(bathymetry[x-1][y]);
            };
            if x < 99 {
                heights.push(bathymetry[x+1][y]);
            };
            if y > 0 {
                heights.push(bathymetry[x][y-1]);
            };
            if y < 99 {
                heights.push(bathymetry[x][y+1]);
            };
            let mut lowest = true;
            for height in heights {
                if bathymetry[x][y] >= height {
                    lowest = false;
                    break;
                }
            }
            if lowest {
                count += bathymetry[x][y] + 1;
            }
        }
    }
    println!("{}", count);
}
