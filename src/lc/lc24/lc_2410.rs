// https://leetcode.com/problems/maximum-matching-of-players-with-trainers/
// 2410. Maximum Matching of Players With Trainers
pub struct Solution;
impl Solution {
    pub fn match_players_and_trainers(players: Vec<i32>, trainers: Vec<i32>) -> i32 {
        let mut players = players;
        let mut trainers = trainers;
        players.sort_unstable();
        trainers.sort_unstable();
        let mut i = 0;
        let mut j = 0;
        let mut ans = 0;
        while i < players.len() && j < trainers.len() {
            if players[i] <= trainers[j] {
                ans += 1;
                i += 1;
                j += 1;
            } else {
                j += 1;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn match_players_and_trainers() {
        assert_eq!(Solution::match_players_and_trainers(vec![4, 7, 9], vec![8, 2, 5, 8]), 2);
        assert_eq!(Solution::match_players_and_trainers(vec![1, 1, 1], vec![10]), 1);
    }
}
