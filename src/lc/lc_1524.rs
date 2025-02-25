// https://leetcode.com/problems/number-of-sub-arrays-with-odd-sum/
// 1524. Number of Sub-arrays With Odd Sum
pub struct Solution;
impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        let mut odd = 0;
        let mut even = 1;
        let mut sum = 0;
        let mut s = 0;
        let m = 1_000_000_007;
        for a in arr {
            s = (s + a) & 1;
            if s == 1 {
                sum = (sum + even) % m;
                odd += 1;
            } else {
                sum = (sum + odd) % m;
                even += 1;
            }
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn num_of_subarrays() {
        assert_eq!(Solution::num_of_subarrays(vec![1, 3, 5]), 4);
        assert_eq!(Solution::num_of_subarrays(vec![2, 4, 6]), 0);
        assert_eq!(Solution::num_of_subarrays(vec![1, 2, 3, 4, 5, 6, 7]), 16);
    }
}
