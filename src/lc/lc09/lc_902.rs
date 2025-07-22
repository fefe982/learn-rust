// https://leetcode.com/problems/numbers-at-most-n-given-digit-set/
// 902. Numbers At Most N Given Digit Set
pub struct Solution;
impl Solution {
    pub fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32 {
        let digits = digits
            .into_iter()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let n = n as i64;
        if n < digits[0] {
            return 0;
        }
        let mut res = 0;
        let base = digits.len() as i32;
        let mut pow10 = 1i64;
        let mut powb = 1;
        let mut last = 1;
        let mut full = 0;
        while n >= pow10 {
            let d = n / pow10 % 10;
            match digits.binary_search(&d) {
                Ok(pos) => last = pos as i32 * powb + last,
                Err(pos) => last = pos as i32 * powb,
            }
            res = full + last;
            pow10 *= 10;
            powb *= base;
            full += powb;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_at_most_n_given_digit_set() {
        assert_eq!(Solution::at_most_n_given_digit_set(vec_str!["5", "8", "9"], 616), 21);
        assert_eq!(Solution::at_most_n_given_digit_set(vec_str!["1", "7"], 231), 10);
        assert_eq!(
            Solution::at_most_n_given_digit_set(vec_str!["3", "4", "5", "6"], 64),
            18
        );
        assert_eq!(Solution::at_most_n_given_digit_set(vec_str!["9"], 55), 1);
        assert_eq!(Solution::at_most_n_given_digit_set(vec_str!["1", "3", "5", "7"], 1), 1);
        assert_eq!(
            Solution::at_most_n_given_digit_set(vec_str!["1", "3", "5", "7"], 100),
            20
        );
        assert_eq!(
            Solution::at_most_n_given_digit_set(vec_str!["1", "4", "9"], 1000000000),
            29523
        );
        assert_eq!(Solution::at_most_n_given_digit_set(vec_str!["7"], 8), 1);
    }
}
