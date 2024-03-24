// https://leetcode.com/problems/stone-game-v/
// 1563. Stone Game V
pub struct Solution;
impl Solution {
    pub fn stone_game_v(stone_value: Vec<i32>) -> i32 {
        let mut sum = vec![0; stone_value.len() + 1];
        let mut dp = vec![vec![0; stone_value.len() + 1]; stone_value.len() + 1];
        for i in 1..=stone_value.len() {
            sum[i] = sum[i - 1] + stone_value[i - 1];
        }
        for i in 2..=stone_value.len() {
            for j in 0..stone_value.len() + 1 - i {
                for k in 1..i {
                    let left = sum[j + k] - sum[j];
                    let right = sum[j + i] - sum[j + k];
                    let v = match (left).cmp(&right) {
                        std::cmp::Ordering::Greater => dp[i - k][j + k] + right,
                        std::cmp::Ordering::Less => dp[k][j] + left,
                        std::cmp::Ordering::Equal => (dp[i - k][j + k]).max(dp[k][j]) + left,
                    };
                    dp[i][j] = v.max(dp[i][j]);
                }
            }
        }
        dp[stone_value.len()][0]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_stone_game_v() {
        assert_eq!(Solution::stone_game_v(vec![6, 2, 3, 4, 5, 5]), 18);
        assert_eq!(Solution::stone_game_v(vec![7, 7, 7, 7, 7, 7, 7]), 28);
        assert_eq!(Solution::stone_game_v(vec![4]), 0);
    }
}
