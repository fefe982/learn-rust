// https://leetcode.com/problems/minimum-size-subarray-in-infinite-array/
// 2875. Minimum Size Subarray Sum
pub struct Solution;
impl Solution {
    pub fn min_size_subarray(nums: Vec<i32>, target: i32) -> i32 {
        let mut cumsum = Vec::with_capacity(nums.len() + 1);
        cumsum.push(0);
        let mut r = std::collections::HashMap::new();
        r.insert(0, 0);
        let mut sum = 0;
        for i in 0..nums.len() {
            sum = sum + nums[i] as i64;
            cumsum.push(sum);
            r.insert(sum, i + 1);
        }
        let target = target as i64;
        let t = target % sum;
        if t == 0 {
            return (target / sum) as i32 * nums.len() as i32;
        }
        let mut ans = i32::MAX;
        for i in 0..nums.len() {
            if let Some(&j) = r.get(&((cumsum[i] + sum - t) % sum)) {
                ans = ans.min(((i + nums.len() - j) % nums.len()) as i32);
            }
        }
        if ans == i32::MAX {
            -1
        } else {
            ans + (target / sum) as i32 * nums.len() as i32
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_size_subarray() {
        assert_eq!(Solution::min_size_subarray(vec![1, 2, 3], 5), 2);
        assert_eq!(Solution::min_size_subarray(vec![1, 1, 1, 2, 3], 4), 2);
        assert_eq!(Solution::min_size_subarray(vec![2, 4, 6, 8], 3), -1);
    }
}
