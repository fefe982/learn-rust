// https://leetcode.com/problems/minimize-maximum-of-array/
// 2439. Minimize Maximum of Array
pub struct Solution;
impl Solution {
    pub fn minimize_array_value(nums: Vec<i32>) -> i32 {
        let mut sum = 0_i64;
        let mut max_val = 0;
        for (&n, idx) in nums.iter().zip(1..) {
            sum += n as i64;
            if sum == 0 {
                continue;
            }
            max_val = std::cmp::max(max_val, (sum - 1) / idx + 1);
        }
        max_val as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimize_array_value() {
        assert_eq!(Solution::minimize_array_value(vec![3, 7, 1, 6]), 5);
        assert_eq!(Solution::minimize_array_value(vec![10, 1]), 10);
    }
}
