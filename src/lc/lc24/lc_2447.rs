// https://leetcode.com/problems/number-of-subarrays-with-gcd-equal-to-k/
// 2447. Number of Subarrays With GCD Equal to K
pub struct Solution;
impl Solution {
    pub fn subarray_gcd(nums: Vec<i32>, k: i32) -> i32 {
        let gcd = |a: i32, b: i32| -> i32 {
            let mut a = a;
            let mut b = b;
            while b != 0 {
                let temp = b;
                b = a % b;
                a = temp;
            }
            a
        };
        let mut cnt = 0;
        for i in 0..nums.len() {
            let mut g = nums[i];
            for j in i..nums.len() {
                g = gcd(g, nums[j]);
                if g == k {
                    cnt += 1;
                } else if g < k {
                    break;
                }
            }
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn subarray_gcd() {
        assert_eq!(Solution::subarray_gcd(vec![9, 3, 1, 2, 6, 3], 3), 4);
        assert_eq!(Solution::subarray_gcd(vec![4], 7), 0);
        assert_eq!(Solution::subarray_gcd(vec![4], 4), 1);
    }
}
