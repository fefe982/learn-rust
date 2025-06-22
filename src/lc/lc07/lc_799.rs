// https://leetcode.com/problems/champagne-tower/
// 799. Champagne Tower
pub struct Solution;
impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let mut dp = vec![0.0; (query_row + 1) as usize];
        dp[0] = poured as f64;
        for i in 0..query_row as usize {
            for j in (0..=i).rev() {
                dp[j] = if dp[j] > 1.0 {
                    (dp[j] - 1.0) / 2.0
                } else {
                    0.0
                };
                dp[j + 1] += dp[j];
            }
        }
        dp[query_glass as usize].min(1.0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_champagne_tower() {
        assert_eq!(Solution::champagne_tower(1, 1, 1), 0.0);
        assert_eq!(Solution::champagne_tower(2, 1, 1), 0.5);
        assert_eq!(Solution::champagne_tower(100000009, 33, 17), 1.0);
    }
}
