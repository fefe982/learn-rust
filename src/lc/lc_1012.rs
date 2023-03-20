// https://leetcode.com/problems/numbers-with-repeated-digits/
// 1012. Numbers With Repeated Digits
pub struct Solution;
use std::collections::HashSet;
impl Solution {
    fn count_num(digits: &Vec<i32>, digit_set: &mut HashSet<i32>, pos: usize) -> i32 {
        let mut cnt = 0;
        let start = if digit_set.is_empty() { 1 } else { 0 };
        if digit_set.is_empty() {
            let mut pow10 = 1;
            let mut c9 = 1;
            let mut c9mul = 9;
            for _ in 1..pos {
                pow10 *= 10;
                c9 *= c9mul;
                cnt += 9 * (pow10 - c9);
                c9mul -= 1;
            }
        }
        for i in start..digits[pos] {
            let mut pow10 = 1;
            let mut cnt_diff = 0;
            for _ in 0..pos {
                pow10 *= 10;
            }
            if digit_set.contains(&i) {
                cnt += pow10;
            } else {
                if cnt_diff == 0 {
                    cnt_diff = 1;
                    let mut left_cnt = (10usize - digit_set.len() - 1) as i32;
                    for _ in 0..pos {
                        cnt_diff *= left_cnt;
                        left_cnt -= 1;
                    }
                }
                cnt += pow10 - cnt_diff;
            }
        }
        if digit_set.contains(&digits[pos]) {
            let mut pow10 = 1;
            for d in digits[0..pos].iter() {
                cnt += *d * pow10;
                pow10 *= 10;
            }
            cnt += 1;
        } else if pos > 0 {
            digit_set.insert(digits[pos]);
            cnt += Self::count_num(digits, digit_set, pos - 1);
        }
        cnt
    }
    pub fn num_dup_digits_at_most_n(mut n: i32) -> i32 {
        if n < 11 {
            return 0;
        }
        let mut digits = vec![0; 10];
        let mut idx = 0usize;
        while n > 0 {
            digits[idx] = n % 10;
            n /= 10;
            idx += 1;
        }
        Self::count_num(&digits, &mut HashSet::new(), idx - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn num_dup_digits_at_most_n() {
        assert_eq!(Solution::num_dup_digits_at_most_n(12), 1);
        assert_eq!(Solution::num_dup_digits_at_most_n(11), 1);
        assert_eq!(Solution::num_dup_digits_at_most_n(20), 1);
        assert_eq!(Solution::num_dup_digits_at_most_n(100), 10);
        assert_eq!(Solution::num_dup_digits_at_most_n(1000), 262);
    }
}
