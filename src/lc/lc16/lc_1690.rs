// https://leetcode.com/problems/stone-game-vii/
// 1690. Stone Game VII
pub struct Solution;
impl Solution {
    pub fn stone_game_vii(stones: Vec<i32>) -> i32 {
        let n = stones.len();
        let mut cumsum = vec![];
        let mut s = 0;
        cumsum.push(0);
        for stone in stones {
            s += stone;
            cumsum.push(s);
        }
        let mut dp = vec![0; n];
        for i in 1..n {
            let mut ndp = vec![0; n - i];
            for j in 0..n - i {
                ndp[j] = (cumsum[j + i + 1] - cumsum[j + 1] - dp[j + 1]).max(cumsum[j + i] - cumsum[j] - dp[j]);
            }
            dp = ndp;
        }
        dp[0]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_stone_game_vii() {
        assert_eq!(Solution::stone_game_vii(vec![5, 3, 1, 4, 2]), 6);
        assert_eq!(Solution::stone_game_vii(vec![7, 90, 5, 1, 100, 10, 10, 2]), 122);
    }
}
