// https://leetcode.com/problems/zigzag-grid-traversal-with-skip/
// 3417. Zigzag Grid Traversal With Skip
pub struct Solution;
impl Solution {
    pub fn zigzag_traversal(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = Vec::new();
        for i in 0..grid.len() {
            if i % 2 == 0 {
                for j in (0..grid[i].len()).step_by(2) {
                    res.push(grid[i][j]);
                }
            } else {
                for j in (0..grid[i].len()).rev().skip(grid[i].len() % 2).step_by(2) {
                    res.push(grid[i][j]);
                }
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn zigzag_traversal() {
        assert_eq!(Solution::zigzag_traversal(vec_vec![[1, 2], [3, 4]]), vec![1, 4]);
        assert_eq!(
            Solution::zigzag_traversal(vec_vec![[2, 1], [2, 1], [2, 1]]),
            vec![2, 1, 2]
        );
        assert_eq!(
            Solution::zigzag_traversal(vec_vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]]),
            vec![1, 3, 5, 7, 9]
        );
    }
}
