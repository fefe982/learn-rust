// https://leetcode.com/problems/alice-and-bob-playing-flower-game/
// 3021. Alice and Bob Playing Flower Game
pub struct Solution;
impl Solution {
    pub fn flower_game(n: i32, m: i32) -> i64 {
        let n = n as i64;
        let m = m as i64;
        (n / 2) * (m / 2 + m % 2) + (n / 2 + n % 2) * (m / 2)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn flower_game() {
        assert_eq!(Solution::flower_game(3, 2), 3);
        assert_eq!(Solution::flower_game(1, 1), 0);
    }
}
