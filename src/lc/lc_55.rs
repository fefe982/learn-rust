// https://leetcode.com/problems/jump-game/
// 55. Jump Game
pub struct Solution;
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max = 0;
        for (i, &n) in nums.iter().enumerate() {
            if i > max {
                return false;
            }
            max = max.max(i + n as usize);
            if max >= nums.len() - 1 {
                return true;
            }
        }
        max >= nums.len() - 1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_jump() {
        assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true);
        assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false);
    }
}
