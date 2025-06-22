// https://leetcode.cn/problems/count-numbers-with-unique-digits/
// 357. Count Numbers with Unique Digits
pub struct Solution;
impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        if n == 0 {
            return 1;
        } else if n == 1 {
            return 10;
        }
        let mut ans = 10;
        let mut num = 9;
        for i in 2..=n {
            num *= 11 - i;
            ans += num;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_numbers_with_unique_digits() {
        assert_eq!(Solution::count_numbers_with_unique_digits(2), 91);
        assert_eq!(Solution::count_numbers_with_unique_digits(0), 1);
    }
}
