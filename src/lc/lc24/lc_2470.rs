// https://leetcode.com/problems/number-of-subarrays-with-lcm-equal-to-k/
// 2470. Number of Subarrays With LCM Equal to K
pub struct Solution;
impl Solution {
    pub fn subarray_lcm(nums: Vec<i32>, k: i32) -> i32 {
        let gcd = |mut a: i32, mut b: i32| {
            while b != 0 {
                let t = b;
                b = a % b;
                a = t;
            }
            a
        };
        let mut ans = 0;
        for i in 0..nums.len() {
            let mut lcm = nums[i];
            for j in i..nums.len() {
                lcm = lcm / gcd(lcm, nums[j]) * nums[j];
                if lcm == k {
                    ans += 1;
                } else if lcm > k {
                    break;
                }
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn subarray_lcm() {
        assert_eq!(Solution::subarray_lcm(vec![3, 6, 2, 7, 1], 6), 4);
        assert_eq!(Solution::subarray_lcm(vec![3], 2), 0);
    }
}
