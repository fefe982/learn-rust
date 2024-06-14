// https://leetcode.com/problems/minimum-increment-to-make-array-unique/
// 945. Minimum Increment to Make Array Unique
pub struct Solution;
impl Solution {
    pub fn min_increment_for_unique(nums: Vec<i32>) -> i32 {
        let mut last = -1;
        let mut ans = 0;
        let mut nums = nums;
        nums.sort_unstable();
        for n in nums {
            last += 1;
            ans += 0.max(last - n);
            last = last.max(n);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_increment_for_unique() {
        assert_eq!(Solution::min_increment_for_unique(vec![1, 2, 2]), 1);
        assert_eq!(Solution::min_increment_for_unique(vec![3, 2, 1, 2, 1, 7]), 6);
    }
}
