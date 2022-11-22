use std::{ fs::read_to_string};
use std::process;

fn main() {
    let numbers_file = read_to_string("numbers.txt").unwrap();
    let boards_file = read_to_string("boards.txt").unwrap();
    
    let bingo_numbers_str: Vec<&str> = numbers_file.split(",").collect();
    let bingo_numbers: Vec<i32> = bingo_numbers_str.iter().map(|n| n.parse().unwrap()).collect();

    let mut i = 0;
    let mut j = 0;
    let mut boards = vec![vec![vec![0; 5]; 5]; 101];
    for line in boards_file.lines() {
        if (i) % 5 == 0 && i != 0 {
            j += 1;
            i = 0;
            continue;
        }
        let row_str: Vec<&str> = line.split_whitespace().collect();
        let row: Vec<i32> = row_str.iter().map(|n| n.parse().unwrap()).collect();
        boards[j][i] = row;
        i += 1;
    }



    for i in 0..bingo_numbers.len() {
        for b in 0..100 {
            // check rows
            for j in 0..5 {
                for k in 0..5 {
                    if !bingo_numbers[0..i].contains(&boards[b][j][k]) {
                        break;
                    }
                    if k == 4 {
                        println!{"BINGO! on board {:?} with numbers {:?} and solution {}", &boards[b], &bingo_numbers[0..i], calculate_remaining_value(&boards[b], &bingo_numbers[0..i])};
                        process::exit(0x0100);
                    }
                }
            }
            // check columns
            for k in 0..5 {
                for j in 0..5 {
                    if !bingo_numbers[0..i].contains(&boards[b][j][k]) {
                        break;
                    }
                    if j == 4 {
                        println!{"BINGO! on board {:?} with numbers {:?} and solution {}", &boards[b], &bingo_numbers[0..i], calculate_remaining_value(&boards[b], &bingo_numbers[0..i])};
                        process::exit(0x0100);
                    }
                }
            }
        }
    }
}

fn calculate_remaining_value(board:  &Vec<Vec<i32>> , numbers:  &[i32] ) -> i32 {
    let mut val = 0;
    for i in 0..5 {
        for j in 0..5 {
            if !numbers.contains(&board[i][j]) {
                val += board[i][j];
                println!("{}", val);
            }
        }
    }
    val *= numbers.last().unwrap();
    return val;
}