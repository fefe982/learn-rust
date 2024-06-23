// https://leetcode.com/problems/longest-continuous-subarray-with-absolute-diff-less-than-or-equal-to-limit/
// 1438. Longest Continuous Subarray With Absolute Diff Less Than or Equal to Limit
pub struct Solution;
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let mut m = std::collections::BTreeMap::<i32, i32>::new();
        let mut i = 0;
        let mut j = 0;
        let mut max = 0;
        while j < nums.len() {
            *m.entry(nums[j]).or_default() += 1;
            j += 1;
            while m.keys().next_back().unwrap() - m.keys().next().unwrap() > limit {
                let c = m.entry(nums[i]).or_default();
                *c -= 1;
                if *c == 0 {
                    m.remove(&nums[i]);
                }
                i += 1;
            }
            max = max.max(j - i);
        }
        max as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_longest_subarray() {
        assert_eq!(Solution::longest_subarray(vec![8, 2, 4, 7], 4), 2);
        assert_eq!(Solution::longest_subarray(vec![10, 1, 2, 4, 7, 2], 5), 4);
        assert_eq!(Solution::longest_subarray(vec![4, 2, 2, 2, 4, 4, 2, 2], 0), 3);
    }
}
