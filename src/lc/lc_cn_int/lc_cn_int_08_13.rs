// https://leetcode.com/problems/pile-box-lcci/
// 面试题 08.13. Pile Box LCCI
pub struct Solution;
impl Solution {
    pub fn pile_box(b: Vec<Vec<i32>>) -> i32 {
        let mut b = b;
        b.sort();
        let mut dp = vec![0; b.len()];
        let mut m = 0;
        for i in 0..b.len() {
            for j in 0..i {
                if b[j][0] < b[i][0] && b[j][1] < b[i][1] && b[j][2] < b[i][2] {
                    dp[i] = dp[i].max(dp[j]);
                }
            }
            dp[i] += b[i][2];
            m = m.max(dp[i]);
        }
        m
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn pile_box() {
        assert_eq!(Solution::pile_box(vec_vec![[1, 1, 1], [2, 2, 2], [3, 3, 3]]), 6);
        assert_eq!(
            Solution::pile_box(vec_vec![[1, 1, 1], [2, 3, 4], [2, 6, 7], [3, 4, 5]]),
            10
        );
    }
}
