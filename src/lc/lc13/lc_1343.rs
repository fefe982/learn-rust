// https://leetcode.com/problems/number-of-sub-arrays-of-size-k-and-average-greater-than-or-equal-to-threshold/
// 1343. Number of Sub-arrays of Size K and Average Greater than or Equal to Threshold
pub struct Solution;
impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let k = k as usize;
        let t = threshold * k as i32;
        let mut sum = 0;
        for &n in arr.iter().take(k) {
            sum += n;
        }
        let mut ans = if sum >= t { 1 } else { 0 };
        for i in k..arr.len() {
            sum += arr[i] - arr[i - k];
            if sum >= t {
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
    fn num_of_subarrays() {
        assert_eq!(Solution::num_of_subarrays(vec![2, 2, 2, 2, 5, 5, 5, 8], 3, 4), 3);
        assert_eq!(
            Solution::num_of_subarrays(vec![11, 13, 17, 23, 29, 31, 7, 5, 2, 3], 3, 5),
            6
        );
    }
}
