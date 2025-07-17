// https://leetcode.com/problems/minimum-difference-in-sums-after-removal-of-elements/
// 2163. Minimum Difference in Sums After Removal of Elements
pub struct Solution;
impl Solution {
    pub fn minimum_difference(nums: Vec<i32>) -> i64 {
        let mut h = std::collections::BinaryHeap::new();
        let n = nums.len() / 3;
        let mut sum = 0;
        for i in 0..n {
            h.push(nums[i]);
            sum += nums[i] as i64;
        }
        let mut sums = vec![0; n + 1];
        sums[0] = sum;
        for i in 0..n {
            h.push(nums[n + i]);
            sum += nums[n + i] as i64;
            sum -= h.pop().unwrap() as i64;
            sums[i + 1] = sum;
        }
        let mut h = std::collections::BinaryHeap::new();
        sum = 0;
        for i in 0..n {
            h.push(std::cmp::Reverse(nums[n * 2 + i]));
            sum += nums[n * 2 + i] as i64;
        }
        let mut ans = sums[n] - sum;
        for i in (0..n).rev() {
            h.push(std::cmp::Reverse(nums[n + i]));
            sum += nums[n + i] as i64;
            sum -= h.pop().unwrap().0 as i64;
            ans = ans.min(sums[i] - sum);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_difference() {
        assert_eq!(Solution::minimum_difference(vec![3, 1, 2]), -1);
        assert_eq!(Solution::minimum_difference(vec![7, 9, 5, 8, 1, 3]), 1);
    }
}
