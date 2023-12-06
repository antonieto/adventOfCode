use super::util::get_path;
use std::fs;

// Pseudo code
// We need a 2d array
// sum = 0;
// for i in nrows:
//    for j in ncols:
//          if arr[i][j] is digit:
//              should_sum = false
//              num = 0
//              row_pointer = j
//              while arr[i][pointer] is digit && pointer <= ncols:
//                  pointer += 1
//                  // Check corners, top, right, bot, left
//                  if arr[i-1][pointer] is_symbol or arr[i-1][pointer + 1] is_symbol...:
//                      should_sum = true
//
//                  // Add to num
//                  num *= 10;
//                  num += arr[i][pointer].to_digit();
//
//                  // Mark cell as visited
//                 arr[i][pointer] = '.';
//
//                 pointer += 1;
//              if should_sum:
//                  sum += num;

const INPUT_PATH: &str = "day3/input.txt";

fn build_arr(source: &str) -> Vec<Vec<char>> {
    let mut arr: Vec<Vec<char>> = Vec::new();
    for line in source.lines() {
        arr.push(line.chars().collect());
    }
    arr
}

fn is_symbol(c: char) -> bool {
    !c.is_digit(10) && c  != '.'
}

struct Coordinate {
    row: usize,
    col: usize,
}

fn adjacent(matrix: &Vec<Vec<char>>, center: Coordinate) -> Vec<Coordinate> {
    let mut result = Vec::new();
    let rows = matrix.len();
    let cols = matrix[0].len();

    // Define the relative positions of adjacent cells (including diagonals)
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1), (1, 0), (1, 1),
    ];

    for (dx, dy) in directions.iter() {
        let new_row = center.row as i32 + dx;
        let new_col = center.col as i32 + dy;

        // Check if the new coordinates are within bounds
        if new_row >= 0 && new_row < rows as i32 && new_col >= 0 && new_col < cols as i32 {
            result.push(Coordinate {
                row: new_row as usize,
                col: new_col as usize,
            });
        }
    }

    result
}


pub fn main() {
    let path = get_path(INPUT_PATH);
    let contents = fs::read_to_string(path).unwrap();
    let mut arr = build_arr(contents.as_str());

    let mut sum = 0;
    let nrows = arr.len();
    let ncols = arr.first().unwrap().len();
    
    for i in 0..=(nrows - 1) {
        for j in 0..=(ncols - 1) {
            if arr[i][j].is_digit(10) {
                let mut should_sum = false;
                let mut num: i32 = 0;
                let mut col_pointer = j;
                while col_pointer < ncols && arr[i][col_pointer].is_digit(10) {
                    // Check around
                    for cell in adjacent(&arr, Coordinate{row: i,col: col_pointer}) {
                        if is_symbol(arr[cell.row][cell.col]) {
                            should_sum = true;
                        }
                    }
                    
                    num *= 10;
                    let digit = arr[i][col_pointer].to_digit(10).unwrap();
                    num += i32::from(digit as i32);
                    

                    arr[i][col_pointer] = '.';
                    
                    col_pointer += 1;
                }
                if should_sum {
                    sum += num;
                }
            }
        }
    }
    println!("Sum: {sum}");
}

