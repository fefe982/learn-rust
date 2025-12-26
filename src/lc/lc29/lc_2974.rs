// https://leetcode.com/problems/minimum-number-game/
// 2974. Minimum Number Game
pub struct Solution;
impl Solution {
    pub fn number_game(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable();
        for i in 0..nums.len() / 2 {
            let t = nums[i * 2];
            nums[i * 2] = nums[i * 2 + 1];
            nums[i * 2 + 1] = t;
        }
        nums
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_number_game() {
        assert_eq!(Solution::number_game(vec![5, 4, 2, 3]), vec![3, 2, 5, 4]);
        assert_eq!(Solution::number_game(vec![2, 5]), vec![5, 2]);
    }
}
