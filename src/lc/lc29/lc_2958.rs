// https://leetcode.com/problems/length-of-longest-subarray-with-at-most-k-frequency/
// 2958. Length of Longest Subarray With at Most K Frequency
pub struct Solution;
impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let mut freq = std::collections::HashMap::<i32, i32>::new();
        let mut i = 0;
        let mut j = 0;
        while j < nums.len() {
            let f = freq.entry(nums[j]).or_default();
            *f += 1;
            if *f <= k {
                ans = ans.max(j - i + 1);
            } else {
                loop {
                    freq.entry(nums[i]).and_modify(|e| *e -= 1);
                    i += 1;
                    if nums[i - 1] == nums[j] {
                        break;
                    }
                }
            }
            j += 1;
        }
        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_subarray_length() {
        assert_eq!(Solution::max_subarray_length(vec![1, 2, 3, 1, 2, 3, 1, 2], 2), 6);
        assert_eq!(Solution::max_subarray_length(vec![1, 2, 1, 2, 1, 2, 1, 2], 1), 2);
        assert_eq!(Solution::max_subarray_length(vec![5, 5, 5, 5, 5, 5, 5], 4), 4);
    }
}
