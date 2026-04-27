// https://leetcode.com/problems/find-greatest-common-divisor-of-array/
// 1979. Find Greatest Common Divisor of Array
pub struct Solution;
impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        for i in nums {
            min = min.min(i);
            max = max.max(i);
        }
        let mut a = min;
        let mut b = max;
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_gcd() {
        assert_eq!(Solution::find_gcd(vec![2, 5, 6, 9, 10]), 2);
        assert_eq!(Solution::find_gcd(vec![7, 5, 6, 8, 3]), 1);
        assert_eq!(Solution::find_gcd(vec![3, 3]), 3);
    }
}
