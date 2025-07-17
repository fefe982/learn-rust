// https://leetcode.com/problems/stone-game/
// 877. Stone Game
pub struct Solution;
impl Solution {
    pub fn stone_game(piles: Vec<i32>) -> bool {
        let mut dp = vec![vec![(0, 0); piles.len()]; piles.len()];
        for i in 0..piles.len() {
            dp[i][i] = (piles[i], 0);
        }
        for i in 1..piles.len() {
            for j in 0..piles.len() - i {
                if piles[j] + dp[j + 1][j + i].1 > piles[j + i] + dp[j][j + i - 1].1 {
                    dp[j][j + i] = (piles[j] + dp[j + 1][j + i].1, dp[j + 1][j + i].0);
                } else {
                    dp[j][j + i] = (piles[j + i] + dp[j][j + i - 1].1, dp[j][j + i - 1].0);
                }
            }
        }
        dp[0][piles.len() - 1].0 > dp[0][piles.len() - 1].1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stone_game() {
        assert_eq!(Solution::stone_game(vec![5, 3, 4, 5]), true);
        assert_eq!(Solution::stone_game(vec![3, 7, 2, 3]), true);
    }
}
