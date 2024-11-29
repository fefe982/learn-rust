// https://leetcode.com/problems/find-if-digit-game-can-be-won/
// 3232. Find if Digit Game Can Be Won
pub struct Solution;
impl Solution {
    pub fn can_alice_win(nums: Vec<i32>) -> bool {
        let (a, b) = nums
            .into_iter()
            .fold((0, 0), |(a, b), x| if x < 10 { (a + x, b) } else { (a, b + x) });
        a != b
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_can_alice_win() {
        assert_eq!(Solution::can_alice_win(vec![5, 5, 5, 15]), false);
        assert_eq!(Solution::can_alice_win(vec![1, 2, 3, 4, 10]), false);
        assert_eq!(Solution::can_alice_win(vec![1, 2, 3, 4, 5, 14]), true);
        assert_eq!(Solution::can_alice_win(vec![5, 5, 5, 25]), true);
    }
}
