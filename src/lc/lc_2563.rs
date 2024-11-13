// https://leetcode.com/problems/count-the-number-of-fair-pairs/
// 2563. Count the Number of Fair Pairs
pub struct Solution;
impl Solution {
    fn count(nums: &Vec<i32>, bound: i32) -> i64 {
        let mut i = 0;
        let mut j = nums.len() - 1;
        let mut cnt = 0;
        while i < j {
            while i < j && nums[i] + nums[j] > bound {
                j -= 1;
            }
            cnt += (j - i) as i64;
            i += 1;
        }
        cnt
    }
    pub fn count_fair_pairs(nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        let mut nums = nums;
        nums.sort_unstable();
        Self::count(&nums, upper) - Self::count(&nums, lower - 1)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_fair_pairs() {
        assert_eq!(Solution::count_fair_pairs(vec![0, 1, 7, 4, 4, 5], 3, 6), 6);
        assert_eq!(Solution::count_fair_pairs(vec![1, 7, 9, 2, 5], 11, 11), 1);
    }
}
