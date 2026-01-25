// https://leetcode.com/problems/maximum-sum-of-edge-values-in-a-graph/
// 3547. Maximum Sum of Edge Values in a Graph
pub struct Solution;
impl Solution {
    pub fn max_score(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        let n = n as i64;
        let ans = (n * n * 2 + 5 * n - 6) * (n - 1) / 6;
        if n as usize == edges.len() {
            ans + 2
        } else {
            ans
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn max_score() {
        assert_eq!(Solution::max_score(4, vec_vec![[0, 1], [1, 2], [2, 3]]), 23);
        assert_eq!(
            Solution::max_score(6, vec_vec![[0, 3], [4, 5], [2, 0], [1, 3], [2, 4], [1, 5]]),
            82
        );
    }
}
