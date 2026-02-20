// https://leetcode.com/problems/sum-of-all-odd-length-subarrays/
// 1588. Sum of All Odd Length Subarrays
pub struct Solution;
impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let mut sum = 0;
        let n = arr.len() as i32;
        for i in 0..n {
            sum += arr[i as usize] * ((i / 2 + 1) * ((n - i - 1) / 2 + 1) + ((i + 1) / 2) * ((n - i) / 2));
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sum_odd_length_subarrays() {
        assert_eq!(Solution::sum_odd_length_subarrays(vec![1, 4, 2, 5, 3]), 58);
        assert_eq!(Solution::sum_odd_length_subarrays(vec![1, 2]), 3);
        assert_eq!(Solution::sum_odd_length_subarrays(vec![10, 11, 12]), 66);
    }
}
