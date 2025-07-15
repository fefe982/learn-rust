// https://leetcode.com/problems/find-the-maximum-length-of-valid-subsequence-ii/
// 3202. Find the Maximum Length of Valid Subsequence II
pub struct Solution;
impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        if nums.len() < 2 {
            return 0;
        }
        let k = k as usize;
        let mut max = 2;
        for m in 0..k {
            let mut v = vec![0; k];
            for i in 0..nums.len() {
                let idx = nums[i] as usize % k;
                v[idx] = v[idx].max(v[(m + k - idx) % k] + 1);
                max = max.max(v[idx]);
            }
        }
        max as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_length() {
        assert_eq!(Solution::maximum_length(vec![1, 2, 3, 4, 5], 2), 5);
        assert_eq!(Solution::maximum_length(vec![1, 4, 2, 3, 1, 4], 3), 4);
    }
}
