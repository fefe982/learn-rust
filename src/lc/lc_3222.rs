// https://leetcode.com/problems/find-the-winning-player-in-coin-game/
// 3222. Find the Winning Player in Coin Game
pub struct Solution;
impl Solution {
    pub fn losing_player(x: i32, y: i32) -> String {
        let n = x.min(y / 4);
        if n % 2 == 0 {
            "Bob".to_string()
        } else {
            "Alice".to_string()
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_losing_player() {
        assert_eq!(Solution::losing_player(2, 7), "Alice".to_string());
        assert_eq!(Solution::losing_player(4, 11), "Bob".to_string());
    }
}
