use std::str::FromStr;
use std::cmp::max;
use std::cmp::min;

#[derive(Debug)]
struct Point {
    x: u64,
    y: u64
}

#[derive(Debug)]
enum Fold {
    X(u64),
    Y(u64),
}

impl FromStr for Point {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(",").unwrap();
        
        return Ok(Point {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        })

    }
}

impl FromStr for Fold {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (axis, value) = s.split_once("fold along ").unwrap().1.split_once("=").unwrap();
        
        return match axis {
            "x" => Ok(Fold::X(value.parse().unwrap())),
            "y" => Ok(Fold::Y(value.parse().unwrap())),
            _ => unreachable!(),
        }
    }
}

fn main() {
    let (points, folds) = include_str!("../input.txt").trim().split_once("\n\n").unwrap();

    let points: Vec<Point> = points.lines().map(str::parse).map(Result::unwrap).collect();
    let folds: Vec<Fold> = folds.lines().map(str::parse).map(Result::unwrap).collect();

    let paper_size = find_paper_size(&points);
    let mut paper = vec![vec![0; paper_size.x as usize]; paper_size.y as usize];
    for p in points {
        paper[p.y as usize][p.x as usize] = 1;
    }
    let mut folded_paper = paper.clone();

    folded_paper = fold(&folded_paper, &folds[0]);

    let mut count = 0;
    for x in 0..folded_paper[0].len(){
        for y in 0..folded_paper.len(){
            count += folded_paper[y][x];
        }
    }
    println!("{}", count);
}

fn fold(paper: &Vec<Vec<i32>>, fold: &Fold) -> Vec<Vec<i32>> {
    match fold {
        Fold::X(x) => x_fold(paper, *x as usize),
        Fold::Y(y) => y_fold(paper, *y as usize),
    }
}

fn y_fold(paper: &Vec<Vec<i32>>, fold_line: usize) -> Vec<Vec<i32>> {
    let mut folded_paper = vec![vec![0; paper[0].len()]; fold_line];
    let add = min(fold_line, paper.len()-fold_line-1) + 1;

    for y in 1..add {
        for x in 0..paper[0].len(){
            folded_paper[fold_line - y][x] = max(paper[fold_line + y][x], paper[fold_line - y][x]);
        }
    }

    return folded_paper;

}

fn x_fold(paper: &Vec<Vec<i32>>, fold_line: usize) -> Vec<Vec<i32>> {
    let mut folded_paper = vec![vec![0; fold_line]; paper.len()];
    let add = min(fold_line, paper[0].len()-fold_line-1) + 1;

    for x in 1..add {
        for y in 0..paper.len(){
            folded_paper[y][fold_line - x] = max(paper[y][fold_line + x],paper[y][fold_line - x]);
        }
    }

    return folded_paper;
}

fn find_paper_size(points: &Vec<Point>) -> Point{
    let mut max_x = 0u64;
    let mut max_y = 0u64;
    for p in points{
        max_x = max(max_x, p.x);
        max_y = max(max_y, p.y);
    }
    return Point{
        x: max_x + 1,
        y: max_y + 1
    }
}