// https://leetcode.com/problems/maximum-subarray-with-equal-products/
// 3411. Maximum Subarray With Equal Products
pub struct Solution;
impl Solution {
    pub fn max_length(nums: Vec<i32>) -> i32 {
        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }
        let mut p = 1;
        let mut i = 0;
        let mut ans = 2;
        for j in 0..nums.len() {
            let nj = nums[j];
            while gcd(p, nj) != 1 {
                p /= nums[i];
                i += 1;
            }
            p *= nj;
            ans = ans.max(j - i + 1);
        }
        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_length() {
        assert_eq!(Solution::max_length(vec![1, 2, 1, 2, 1, 1, 1]), 5);
        assert_eq!(Solution::max_length(vec![2, 3, 4, 5, 6]), 3);
        assert_eq!(Solution::max_length(vec![1, 2, 3, 1, 4, 5, 1]), 5);
    }
}
