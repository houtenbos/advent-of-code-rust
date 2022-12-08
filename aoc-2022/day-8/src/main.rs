use std::fs;
const UP: (isize,  isize) = (-1, 0);
const DOWN: (isize, isize) = (1, 0);
const LEFT: (isize, isize) = (0, -1);
const RIGHT: (isize, isize) = (0, 1);

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let size = 99;

    let mut trees = vec![vec![0; size]; size];
    let mut visibility_map = vec![vec![false; size]; size];
    visibility_map[trees.len()-1] = vec![true; size];
    for i in 0..trees.len() {
        visibility_map[i][trees[i].len()-1] = true;
    }

    for (i, line) in file.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            trees[i][j] = c.to_digit(10).unwrap();
        }
    }

    // a
    let mut counter = 0;
    for i in 0..trees.len(){
        for j in 0..trees[i].len() {
            let height = trees[i][j];
            if check_visibility(&trees, height, i, j, UP) {counter += 1}
            else if check_visibility(&trees, height, i, j, DOWN) {counter += 1}
            else if check_visibility(&trees, height, i, j, LEFT) {counter += 1}
            else if check_visibility(&trees, height, i, j, RIGHT) {counter += 1}
        }
    }
    println!("{}", counter);

    // b
    let mut high_score = 0;
    for i in 0..trees.len(){
        for j in 0..trees[i].len() {
            let height = trees[i][j];
            let mut score = 1;
            score *= get_score(&trees, height, i, j, UP);
            score *= get_score(&trees, height, i, j, DOWN);
            score *= get_score(&trees, height, i, j, RIGHT);
            score *= get_score(&trees, height, i, j, LEFT);
            if score > high_score {
                high_score = score;
            }
        }
    }
    println!("{}", high_score);
}


fn check_visibility(trees: &Vec<Vec<u32>>, height: u32, i : usize, j: usize, direction: (isize, isize) ) -> bool{
    if i == 0 || j == 0 || i == trees.len() -1 || j == trees[i].len() -1 {
        return true;
    }

    let x = (i as isize + direction.0) as usize;
    let y = (j as isize + direction.1) as usize;

    if trees[x][y] >= height {
        return false;
    }else{
        return check_visibility(trees, height, x, y, direction);
    }
}

fn get_score(trees: &Vec<Vec<u32>>, height: u32, i : usize, j: usize, direction: (isize, isize) ) -> i32{
    if i == 0 || j == 0 || i == trees.len() -1 || j == trees[i].len() -1 {
        return 0;
    }

    let x = (i as isize + direction.0) as usize;
    let y = (j as isize + direction.1) as usize;

    if trees[x][y] >= height {
        return 1;
    }else{
        return 1+get_score(trees, height, x, y, direction);
    }
}
