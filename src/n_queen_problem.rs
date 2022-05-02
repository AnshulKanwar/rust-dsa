fn is_safe(board: &Vec<Vec<bool>>, x: usize, y: usize) -> bool {
    let n = board.len();

    // if same column or row
    for i in 0..n {
        if board[x][i] || board[i][y] {
            return false;
        }
    }

    // upper left diagonal
    let mut row = x as i32;
    let mut col = y as i32;

    while row >= 0 && col >= 0 {
        if board[row as usize][col as usize] {
            return false;
        }
        row -= 1;
        col -= 1;
    }

    // lower right diagonal
    let mut row = x as i32;
    let mut col = y as i32;

    while row < n as i32 && col < n as i32 {
        if board[row as usize][col as usize] {
            return false;
        }
        row += 1;
        col += 1;
    }

    // upper right diagonal
    let mut row = x as i32;
    let mut col = y as i32;

    while row >= 0 as i32 && col < n as i32 {
        if board[row as usize][col as usize] {
            return false;
        }
        row -= 1;
        col += 1;
    }

    // lower left diagonal
    let mut row = x as i32;
    let mut col = y as i32;

    while row < n as i32 && col >= 0 as i32 {
        if board[row as usize][col as usize] {
            return false;
        }
        row += 1;
        col -= 1;
    }

    return true;
}

fn solver(board: &mut Vec<Vec<bool>>, col: usize) -> bool {
    let n = board.len();

    if col >= n {
        return true;
    }

    for i in 0..n {
        // Check if (i, col) is safe
        if is_safe(&board, i, col) {
            // if yes, then place queen there
            board[i][col] = true;

            // Recursively check if there exists a place to put next queen
            if solver(board, col + 1) {
                return true;
            } else {
                board[i][col] = false;
            }
        }
    }

    return false;
}

pub fn n_queen_problem(n: usize) -> Vec<Vec<bool>> {
    let mut solution = vec![vec![false; n]; n];

    solver(&mut solution, 0);

    solution
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_col() {
        let n = 4;
        let mut board = vec![vec![false; n]; n];

        board[1][1] = true;

        assert!(!is_safe(&board, 0, 1));
        assert!(!is_safe(&board, 2, 1));
        assert!(!is_safe(&board, 3, 1));
    }

    #[test]
    fn same_row() {
        let n = 4;
        let mut board = vec![vec![false; n]; n];

        board[1][1] = true;

        assert!(!is_safe(&board, 1, 0));
        assert!(!is_safe(&board, 1, 2));
        assert!(!is_safe(&board, 1, 3));
    }

    #[test]
    fn negative_diagonal() {
        let n = 4;
        let mut board = vec![vec![false; n]; n];

        board[1][1] = true;

        assert!(!is_safe(&board, 0, 0));
        assert!(!is_safe(&board, 2, 2));
        assert!(!is_safe(&board, 3, 3));

        board[1][1] = false;
        board[2][1] = true;

        assert!(!is_safe(&board, 1, 0));
        assert!(!is_safe(&board, 3, 2));
    }

    #[test]
    fn positive_diagonal() {
        let n = 4;
        let mut board = vec![vec![false; n]; n];

        board[1][1] = true;

        assert!(!is_safe(&board, 0, 2));
        assert!(!is_safe(&board, 2, 0));

        board[1][1] = false;
        board[2][1] = true;

        assert!(!is_safe(&board, 0, 3));
        assert!(!is_safe(&board, 1, 2));
        assert!(!is_safe(&board, 3, 0));
    }

    #[test]
    fn safe() {
        let n = 4;
        let mut board = vec![vec![false; n]; n];

        board[1][1] = true;

        assert!(is_safe(&board, 0, 3));
        assert!(is_safe(&board, 2, 3));
        assert!(is_safe(&board, 3, 0));
        assert!(is_safe(&board, 3, 2));

        board[1][1] = false;
        board[2][1] = true;

        assert!(is_safe(&board, 0, 0));
        assert!(is_safe(&board, 0, 2));
        assert!(is_safe(&board, 1, 3));
        assert!(is_safe(&board, 3, 3));
    }
}
