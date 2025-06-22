// https://leetcode.com/problems/n-queens-ii/
// 52. N-Queens II
pub struct Solution;
impl Solution {
    fn check(s: &Vec<i32>, idx: usize) -> bool {
        if idx == 0 {
            return true;
        }
        for i in 0..idx {
            if s[i] == s[idx] {
                return false;
            }
            if (idx - i) as i32 == (s[i] - s[idx]).abs() {
                return false;
            }
        }
        true
    }
    pub fn total_n_queens(n: i32) -> i32 {
        let n = n as usize;
        let mut solv = vec![0; n];
        let mut cnt = 0;
        let mut col = 0;
        let mut row = 0;

        loop {
            solv[col] = row;
            if Self::check(&solv, col) {
                if col == n - 1 {
                    cnt += 1;
                    row += 1;
                } else {
                    col += 1;
                    row = 0;
                }
            } else {
                row += 1;
            }
            while row == n as i32 {
                if col == 0 {
                    return cnt;
                }
                col -= 1;
                row = solv[col] + 1;
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn total_n_queens() {
        assert_eq!(Solution::total_n_queens(4), 2);
        assert_eq!(Solution::total_n_queens(1), 1);
    }
}
