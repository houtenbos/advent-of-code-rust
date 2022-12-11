use std::fs;
use std::collections::HashSet;

struct Location {
    x: i32,
    y: i32
}

impl Location {
    fn new () -> Location {
        Location{x: 0, y: 0}
    }
}

struct Knot {
    location: Location,
    tail: Option<Box<Knot>>,
}

impl Knot {
    fn new(loc: Location) -> Knot {
        Knot {
            location: loc,
            tail: None,
        }
    }
    fn move_knot(&mut self, direction: &Location){
        self.location.x += direction.x;
        self.location.y += direction.y;
    }
}

struct Rope {
    head: Option<Box<Knot>>,
}

impl Rope {
    fn new() -> Rope {
        Rope { head: None }
    }

    fn append(&mut self, data: Location) {
        let new_node = Box::new(Knot::new(data));

        let mut current = &mut self.head;
        while let Some(ref mut node) = current {
            current = &mut node.tail;
        }
        *current = Some(new_node);
    }

    fn move_head(&mut self, direction: &Location){
        let current = &mut self.head;
        match current {
            Some(ref mut knot) => {knot.move_knot(direction)},
            None => {}
        }

        self.move_tail();
    }

    fn move_tail(&mut self){
        let mut head = &mut self.head;
        while let Some(ref mut knot) = head {
            // check if this knot has a tail
            let tail = if let Some(ref mut tail) = knot.tail {
                tail
            }
                else { 
                // reached the end of the rope
                // println!("moved tail to {} {}", knot.location.x, knot.location.y);
                break;
            };

            // calculate the direction the tail needs to move to
            let dx = knot.location.x - tail.location.x;
            let dy = knot.location.y - tail.location.y;
            let mut dir = Location{x: 0, y: 0};
            if i32::abs(dx) <= 1 && i32::abs(dy) <= 1 {
                // do nothing
            }else if dx == 0 {
                // move vertical
                dir.y += 1 * dy/i32::abs(dy);
            }else if dy == 0 {
                // move horizontal
                dir.x += 1 * dx/i32::abs(dx);
            }else{
                // move diagonal
                dir.x += 1 * dx/i32::abs(dx);
                dir.y += 1 * dy/i32::abs(dy);
            }

            // move the tail in the correct direction
            tail.move_knot(&dir);

            // get next knot
            head = &mut knot.tail;
        }
    }

    fn get_tail(&self) -> Option<&Box<Knot>> {
        let mut current = &self.head;
        while let Some(ref knot) = current {
            if knot.tail.is_none() {
                return Some(knot)
            }
            current = &knot.tail
        }
        None
    }
}


fn main() {
    let mut rope = Rope::new();
    let mut locations = HashSet::new();
    for _ in 0..10 {
        rope.append(Location::new());
    }

    let file = fs::read_to_string("input.txt").unwrap();
    for line in file.lines(){
        let moves: Vec<&str> = line.split(" ").collect();
        let dir = moves[0];
        let steps: u32 = moves[1].parse().unwrap();
        for _ in 0..steps {
            match dir.chars().last().unwrap() {
                'R' => rope.move_head(&Location{x: 1, y:0}),
                'L' => rope.move_head(&Location{x: -1, y:0}),
                'U' => rope.move_head(&Location{x: 0, y:1}),
                'D' => rope.move_head(&Location{x: 0, y:-1}),
                _ => println!("Invalid direction")
            }

            // store the location of the tail
            let tail = rope.get_tail().unwrap();
            let loc = format!("{},{}", tail.location.x, tail.location.y);
            locations.insert(loc);
        }
    }
    println!("Tail was in {} different locations", locations.len());
}
