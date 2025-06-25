// https://leetcode.com/problems/number-of-zero-filled-subarrays/
// 2348. Number of Zero-Filled Subarrays
pub struct Solution;
impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let mut total = 0i64;
        let mut last0 = -1;
        for (&n, idx) in nums.iter().chain([1].iter()).zip(0..) {
            match n {
                0 => {
                    if last0 == -1 {
                        last0 = idx;
                    }
                }
                _ => {
                    if last0 != -1 {
                        let n = (idx - last0) as i64;
                        total += n * (n + 1) / 2;
                    }
                    last0 = -1;
                }
            }
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn zero_filled_subarray() {
        assert_eq!(
            Solution::zero_filled_subarray(vec![1, 3, 0, 0, 2, 0, 0, 4]),
            6
        );
        assert_eq!(Solution::zero_filled_subarray(vec![0, 0, 0, 2, 0, 0]), 9);
        assert_eq!(Solution::zero_filled_subarray(vec![2, 10, 2019]), 0);
    }
}
