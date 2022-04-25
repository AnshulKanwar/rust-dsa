use std::cmp::max;

pub fn longest_common_subsequence(string1: &str, string2: &str) -> Vec<Vec<u32>> {
    let m = string1.len();
    let n = string2.len();
    let mut solution = vec![vec![0; n]; m];

    for i in 1..m {
        for j in 1..n {
            if string1.as_bytes()[i] == string2.as_bytes()[j] {
                solution[i][j] = 1 + solution[i - 1][j - 1];
            } else {
                solution[i][j] = max(solution[i - 1][j], solution[i][j - 1]);
            }
        }
    }

    solution
}
