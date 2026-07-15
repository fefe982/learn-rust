// https://leetcode.com/problems/sum-of-gcd-of-formed-pairs/
// 3867. Sum of GCD of Formed Pairs
pub struct Solution;
impl Solution {
    pub fn gcd_sum(nums: Vec<i32>) -> i64 {
        let mut nums = nums;
        let mut mx = 0;
        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 {
                return a;
            }
            gcd(b, a % b)
        }
        for i in 0..nums.len() {
            mx = mx.max(nums[i]);
            nums[i] = gcd(mx, nums[i]);
        }
        nums.sort_unstable();
        let mut i = 0;
        let mut j = nums.len() - 1;
        let mut res = 0;
        while i < j {
            res += gcd(nums[i], nums[j]) as i64;
            i += 1;
            j -= 1;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gcd_sum() {
        assert_eq!(Solution::gcd_sum(vec![2, 6, 4]), 2);
        assert_eq!(Solution::gcd_sum(vec![3, 6, 2, 8]), 5);
    }
}
