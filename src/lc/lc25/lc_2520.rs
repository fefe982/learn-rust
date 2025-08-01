// https://leetcode.com/problems/count-the-digits-that-divide-a-number/
// 2520. Count the Digits That Divide a Number
pub struct Solution;
impl Solution {
    pub fn count_digits(num: i32) -> i32 {
        let mut n = num;
        let mut cnt = 0;
        while n > 0 {
            let d = n % 10;
            if d > 0 && num % d == 0 {
                cnt += 1;
            }
            n = n / 10;
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_digits() {
        assert_eq!(Solution::count_digits(7), 1);
        assert_eq!(Solution::count_digits(121), 2);
        assert_eq!(Solution::count_digits(1248), 4);
    }
}
