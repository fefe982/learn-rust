// https://leetcode.com/problems/longest-square-streak-in-an-array/
// 2501. Longest Square Streak in an Array
pub struct Solution;
impl Solution {
    pub fn longest_square_streak(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut cnt = vec![0; nums.len()];
        let mut max = 0;
        let mut i = 0;
        let mut j = nums.partition_point(|&x| nums[0] > x / nums[0]);
        while j < nums.len() {
            while j < nums.len() && nums[i] > nums[j] / nums[i] {
                j += 1;
            }
            while j < nums.len() && nums[i] == nums[j] / nums[i] && nums[j] % nums[i] == 0 {
                cnt[j] = cnt[i] + 1;
                max = max.max(cnt[j]);
                j += 1;
            }
            i += 1;
        }
        if max == 0 {
            -1
        } else {
            max + 1
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_longest_square_streak() {
        assert_eq!(Solution::longest_square_streak(vec![48841, 49]), -1);
        assert_eq!(Solution::longest_square_streak(vec![4, 3, 6, 16, 8, 2]), 3);
        assert_eq!(Solution::longest_square_streak(vec![2, 3, 5, 6, 7]), -1);
    }
}
