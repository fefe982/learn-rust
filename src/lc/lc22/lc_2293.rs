// https://leetcode.com/problems/min-max-game/
// 2293. Min Max Game
pub struct Solution;
impl Solution {
    pub fn min_max_game(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut n = nums.len();
        while n > 1 {
            for i in (0..n).step_by(2) {
                if i % 4 == 0 {
                    nums[i / 2] = nums[i].min(nums[i + 1]);
                } else {
                    nums[i / 2] = nums[i].max(nums[i + 1]);
                }
            }
            n /= 2;
        }
        nums[0]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_max_game() {
        assert_eq!(Solution::min_max_game(vec![1, 3, 5, 2, 4, 8, 2, 2]), 1);
        assert_eq!(Solution::min_max_game(vec![3]), 3);
    }
}
