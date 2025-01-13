use std::ops::Index;
use crate::find_all_anagrams_a_string_438::find_anagrams;

pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let row = matrix.len();
    let column = matrix[0].len();
    let mut target: Vec<usize> = Vec::new();
    for i in 0..row {
        for j in 0..column {
            if matrix[i][j] == 0 {
                target.push(i);
                target.push(j);
            }
        }
    }

    for i in (0..target.len()).step_by(2) {
        let cur_row = target[i];
        let cur_column = target[i + 1];
        for j in 0..column {
            matrix[cur_row][j] = 0;
        }
        for j in 0..row {
            matrix[j][cur_column] = 0;
        }
    }
}
pub fn test() {
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    matrix.push(vec![1, 1, 1]);
    matrix.push(vec![1, 0, 1]);
    matrix.push(vec![1, 1, 1]);
    set_zeroes(&mut matrix);
    println!("{:?}", matrix);
}