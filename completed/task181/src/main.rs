/*
    https://www.codewars.com/kata/525caa5c1bf619d28c000335/train/rust
 */

fn is_solved(board: &[&[u8; 3]; 3]) -> i8 {
    let mut is_no_more_moves = 0;
    if board[0][0] == board[1][1] && board[1][1] == board[2][2] && board[0][0] == board[2][2] && board[0][0] != 0 {
        return board[0][0] as i8;
    }
    if board[0][2] == board[1][1] && board[1][1] == board[2][0] && board[0][2] == board[2][0] && board[0][2] != 0 {
        return board[0][2] as i8;
    }
    for i in 0..3 {
        if board[i][0] == board[i][1] && board[i][1] == board[i][2] && board[i][0] == board[i][2] && board[i][0] != 0 {
            return board[i][0] as i8;
        }
        if board[0][i] == board[1][i] && board[2][i] == board[2][i] && board[0][i] == board[2][i] && board[0][i] != 0 {
            return board[0][i] as i8;
        }
        if board[i][0] == 0 || board[i][1] == 0 || board[i][2] == 0 {
            is_no_more_moves = -1;
        }
    }
    is_no_more_moves
}

fn main() {
    println!("{}", is_solved(&[&[0, 0, 1], &[0, 1, 2], &[2, 1, 0]]));
    println!("{}", is_solved(&[&[1, 1, 1], &[0, 2, 2], &[0, 0, 0]]));
    println!("{}", is_solved(&[&[2, 1, 2], &[2, 1, 1], &[1, 1, 2]]));
    println!("{}", is_solved(&[&[2, 1, 2], &[2, 1, 1], &[1, 2, 1]]));
}
