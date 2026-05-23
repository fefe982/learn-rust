// https://leetcode.com/problems/count-integers-with-even-digit-sum/
// 2180. Count Integers With Even Digit Sum
pub struct Solution;
impl Solution {
    pub fn count_even(num: i32) -> i32 {
        let mut n = num;
        let mut s = 0;
        while n > 0 {
            s += n % 10;
            n /= 10;
        }
        if s % 2 == 0 {
            num / 2
        } else {
            (num - 1) / 2
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_even() {
        assert_eq!(Solution::count_even(4), 2);
        assert_eq!(Solution::count_even(30), 14);
    }
}
