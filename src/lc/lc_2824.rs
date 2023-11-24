// https://leetcode.com/problems/count-pairs-whose-sum-is-less-than-target/
// 2824. Count Pairs Whose Sum is Less than Target
pub struct Solution;
impl Solution {
    pub fn count_pairs(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut i = 0;
        let mut j = 1;
        while j < nums.len() && nums[i] + nums[j] < target {
            j += 1;
        }
        let mut count = 0;
        while i < j {
            count += j - i - 1;
            i += 1;
            while i < j - 1 && nums[i] + nums[j - 1] >= target {
                j -= 1;
            }
        }
        count as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_pairs() {
        assert_eq!(Solution::count_pairs(vec![-1, 1, 2, 3, 1], 2), 3);
        assert_eq!(Solution::count_pairs(vec![-6, 2, 5, -2, -7, -1, 3], -2), 10);
    }
}
