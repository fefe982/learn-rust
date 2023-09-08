// https://leetcode.com/problems/pascals-triangle/
// 118. Pascal's Triangle
pub struct Solution;
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![vec![1]];
        for i in 1..num_rows as usize {
            let mut row = vec![1];
            for j in 1..i {
                row.push(ans[i - 1][j - 1] + ans[i - 1][j]);
            }
            row.push(1);
            ans.push(row);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn generate() {
        assert_eq!(
            Solution::generate(5),
            vec_vec![[1], [1, 1], [1, 2, 1], [1, 3, 3, 1], [1, 4, 6, 4, 1]]
        );
        assert_eq!(Solution::generate(1), vec_vec![[1]]);
    }
}
