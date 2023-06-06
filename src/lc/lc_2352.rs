// https://leetcode.com/problems/equal-row-and-column-pairs/
// 2352. Equal Row and Column Pairs
pub struct Solution;
impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut num = 0;
        for i in 0..n {
            for j in 0..n {
                if 'good: loop {
                    for k in 0..n {
                        if grid[i][k] != grid[k][j] {
                            break 'good false;
                        }
                    }
                    break 'good true;
                } {
                    num += 1;
                }
            }
        }
        num
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn equal_pairs() {
        assert_eq!(
            Solution::equal_pairs(vec_vec![[3, 2, 1], [1, 7, 6], [2, 7, 7]]),
            1
        );
        assert_eq!(
            Solution::equal_pairs(vec_vec![
                [3, 1, 2, 2],
                [1, 4, 4, 5],
                [2, 4, 2, 2],
                [2, 4, 2, 2]
            ]),
            3
        );
    }
}
