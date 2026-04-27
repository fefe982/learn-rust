// https://leetcode.com/problems/minimize-the-difference-between-target-and-chosen-elements
// 1981. Minimize the Difference Between Target and Chosen Elements
pub struct Solution;
impl Solution {
    pub fn minimize_the_difference(mat: Vec<Vec<i32>>, target: i32) -> i32 {
        let mut dp = vec![false; 5001];
        dp[0] = true;
        for row in mat {
            let mut next = vec![false; 5001];
            for &x in &row {
                for i in 0..=5000 - x as usize {
                    if dp[i] {
                        next[i + x as usize] = true;
                    }
                }
            }
            dp = next;
        }
        let mut ans = i32::MAX;
        for i in 0..=5000 {
            if dp[i] {
                ans = ans.min((i as i32 - target).abs());
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_minimize_the_difference() {
        assert_eq!(
            Solution::minimize_the_difference(vec_vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]], 13),
            0
        );
        assert_eq!(Solution::minimize_the_difference(vec_vec![[1], [2], [3]], 100), 94);
        assert_eq!(Solution::minimize_the_difference(vec_vec![[1, 2, 9, 8, 7]], 6), 1);
    }
}
