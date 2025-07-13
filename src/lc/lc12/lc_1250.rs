// https://leetcode.com/problems/check-if-it-is-a-good-array/
// 1250. Check If It Is a Good Array
pub struct Solution;
impl Solution {
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        loop {
            if a == 0 {
                return b;
            }
            b = b % a;
            if b == 0 {
                return a;
            }
            a = a % b;
        }
    }
    pub fn is_good_array(nums: Vec<i32>) -> bool {
        let mut g = nums[0];
        if g == 1 {
            return true;
        }
        for n in nums.into_iter().skip(1) {
            g = Self::gcd(g, n);
            if g == 1 {
                return true;
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_good_array() {
        assert_eq!(Solution::is_good_array(vec![12, 5, 7, 23]), true);
        assert_eq!(Solution::is_good_array(vec![29, 6, 10]), true);
        assert_eq!(Solution::is_good_array(vec![3, 6]), false);
    }
}
