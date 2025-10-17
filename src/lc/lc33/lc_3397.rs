// https://leetcode.com/problems/maximum-number-of-distinct-elements-after-operations/
// 3397. Maximum Number of Distinct Elements After Operations
pub struct Solution;
impl Solution {
    pub fn max_distinct_elements(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut c = nums[0] - k;
        let mut ans = 1;
        for i in 1..nums.len() {
            if nums[i] - k > c {
                ans += 1;
                c = nums[i] - k;
            } else if nums[i] + k <= c {
                continue;
            } else {
                c = c + 1;
                ans += 1;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_distinct_elements() {
        assert_eq!(Solution::max_distinct_elements(vec![1, 2, 2, 3, 3, 4], 2), 6);
        assert_eq!(Solution::max_distinct_elements(vec![4, 4, 4, 4], 1), 3);
    }
}
