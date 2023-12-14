// https://leetcode.com/problems/chalkboard-xor-game/
// 810. Chalkboard XOR Game
pub struct Solution;
impl Solution {
    pub fn xor_game(nums: Vec<i32>) -> bool {
        nums.iter().fold(0, |acc, &n| acc ^ n) == 0 || nums.len() % 2 == 0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_xor_game() {
        assert_eq!(Solution::xor_game(vec![1, 1, 2]), false);
        assert_eq!(Solution::xor_game(vec![0, 1]), true);
        assert_eq!(Solution::xor_game(vec![1, 2, 3]), true);
    }
}
