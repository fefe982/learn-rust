// https://leetcode.com/problems/count-good-numbers/
// 1922. Count Good Numbers
pub struct Solution;
impl Solution {
    pub fn count_good_numbers(n: i32) -> i32 {
        let m = 1_000_000_007i64;
        let mut r = 1;
        let mut b = (n / 2) as i64;
        let mut a = 20;
        while b > 0 {
            if b & 1 == 1 {
                r = (r * a) % m;
            }
            a = (a * a) % m;
            b >>= 1;
        }
        if n % 2 == 1 {
            r = (r * 5) % m;
        }
        r as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_good_numbers() {
        assert_eq!(Solution::count_good_numbers(1), 5);
        assert_eq!(Solution::count_good_numbers(4), 400);
        assert_eq!(Solution::count_good_numbers(50), 564908303);
    }
}
