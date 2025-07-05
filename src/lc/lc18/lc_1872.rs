// https://leetcode.com/problems/stone-game-viii/
// 1872. Stone Game VIII
pub struct Solution;
impl Solution {
    pub fn stone_game_viii(stones: Vec<i32>) -> i32 {
        let n = stones.len();
        let mut sum = stones.iter().sum::<i32>();
        let mut ans = sum;
        for i in (2..n).rev() {
            sum -= stones[i];
            ans = ans.max(sum - ans);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_stone_game_viii() {
        assert_eq!(Solution::stone_game_viii(vec![-7, 25, -36, -32, 17, -42, -5, -11]), 11);
        assert_eq!(
            Solution::stone_game_viii(vec![-39, -23, -43, -7, 25, -36, -32, 17, -42, -5, -11]),
            11
        );
        assert_eq!(
            Solution::stone_game_viii(vec![7, -1, 29, -4, -41, 3, 31, 22, -12, -29, 50, 47, -44, -5, -6, 3, 5]),
            55
        );
        assert_eq!(Solution::stone_game_viii(vec![-10, -12, -10, -12]), 12);
        assert_eq!(Solution::stone_game_viii(vec![-1, 2, -3, 4, -5]), 5);
        assert_eq!(Solution::stone_game_viii(vec![7, -6, 5, 10, 5, -2, -6]), 13);
        assert_eq!(Solution::stone_game_viii(vec![-10, -12]), -22);
    }
}
