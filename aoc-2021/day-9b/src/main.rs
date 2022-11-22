use std::fs::read_to_string;

const WIDTH: usize = 100;
const HEIGHT: usize = 100;
const INPUT: &str = "input.txt";

struct Coordinate {
    x: u32,
    y: u32
}
struct Basin {
    coordinates: Vec<Coordinate>
}
impl Basin {
    fn new() -> Basin {
        Basin {coordinates: vec![]}
    }
    fn expore(&mut self, bathymetry: &Vec<Vec<u32>>, coordinate: &Coordinate) {
        if self.is_empty() || !self.includes(coordinate.x, coordinate.y) {
            self.add(coordinate.x, coordinate.y);
        }
        else{
            return
        }

        let mut coordinates = vec![];
        if coordinate.x > 0 {
            coordinates.push(Coordinate{x: coordinate.x-1, y: coordinate.y});
        };
        if coordinate.x + 1 < WIDTH as u32 {
            coordinates.push(Coordinate{x: coordinate.x+1, y: coordinate.y});
        };
        if coordinate.y > 0 {
            coordinates.push(Coordinate{x: coordinate.x, y: coordinate.y -1});
        };
        if coordinate.y + 1 < HEIGHT as u32 {
            coordinates.push(Coordinate{x: coordinate.x, y: coordinate.y + 1});
        };

        for c in coordinates {
            if bathymetry[c.y as usize][c.x as usize] != 9u32 && !self.includes(c.x, c.y) {
                self.expore(bathymetry, &c)
            }
        }

    }
    fn includes(&self, x: u32, y: u32) -> bool {
        let mut result = false;
        for coordinate in &self.coordinates {
            if coordinate.x == x && coordinate.y == y {
                result = true;
                break;
            }
        }
        result
    }
    fn is_empty(&self) -> bool {
        let result = self.coordinates.is_empty();
        result
    }
    fn add(&mut self, x: u32, y: u32) {
        self.coordinates.push(Coordinate{x:x, y:y})
    }
    fn size(&self) -> u32 {
        let result = self.coordinates.len() as u32;
        result
    }
}

fn already_in_basin(basins: &Vec<Basin>, x: u32, y: u32) -> bool {
    let mut result = false;
    for basin in basins {
        if basin.includes(x as u32, y as u32) {
            result = true;
            break;
        }
    }
    result
}

fn main() {
    let input = read_to_string(INPUT).unwrap();


    let mut bathymetry = vec![vec![0; WIDTH]; HEIGHT];
    let mut i = 0;
    for line in input.lines() {
        bathymetry[i] = line.chars().map(|n| n.to_digit(10).unwrap()).collect::<Vec<u32>>();
        i += 1;
    };
    let mut basins: Vec<Basin> = vec![];
    let mut top_3 = [0u32; 3];
    let mut full = [false; HEIGHT];

    loop {
        let mut basin = Basin::new();
        for y in 0..HEIGHT as u32 {
            if full[y as usize] {
                continue;
            }
            for x in 0..WIDTH as u32 {
                let height = bathymetry[y as usize][x as usize];
                let coordinate = Coordinate{x: x, y: y};
                if height == 9 {
                    continue;
                }
                if already_in_basin(&basins, x, y){
                    continue;
                }
                if basin.is_empty() {
                    basin.expore(&bathymetry, &coordinate);
                    break;
                }
            }
            if !basin.is_empty() {
                break;
            }
            else{
                full[y as usize] = true;
            }
        }
        if basin.is_empty(){
            break;
        }
        else{
            let size = basin.size().clone();
            i += 1;
            basins.push(basin);
            if size > top_3[0] {
                top_3[2] = top_3[1];
                top_3[1] = top_3[0];
                top_3[0] = size;
            } else if size > top_3[1] {
                top_3[2] = top_3[1];
                top_3[1] = size;
            } else if size > top_3[2] {
                top_3[2] = size;
            }
        };
    }
    // add up all basin sizes
    println!("{:?} {}", top_3, top_3[0] * top_3[1] * top_3[2]);

}
