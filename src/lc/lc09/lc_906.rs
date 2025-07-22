// https://leetcode.com/problems/super-palindromes/
// 906. Super Palindromes
pub struct Solution;
impl Solution {
    fn make_pal_even(n: i64) -> i64 {
        let mut m = n;
        let mut n = n;
        while n > 0 {
            m = m * 10 + n % 10;
            n /= 10;
        }
        m
    }
    fn make_pal_odd(n: i64) -> i64 {
        let mut m = n / 10;
        let mut n = n;
        while n > 0 {
            m = m * 10 + n % 10;
            n /= 10;
        }
        m
    }
    fn check_pal(n: i64) -> bool {
        let mut m = 0;
        let mut n = n;
        while n > m {
            if n / 10 == m {
                return true;
            }
            m = m * 10 + n % 10;
            n /= 10;
        }
        n == m
    }
    pub fn superpalindromes_in_range(left: String, right: String) -> i32 {
        let left = left.parse::<i64>().unwrap();
        let right = right.parse::<i64>().unwrap();
        let mut i = 1;
        let mut cnt = 0;
        loop {
            let pal = Self::make_pal_odd(i);
            let mul = pal * pal;
            i += 1;
            if mul < left {
                continue;
            }
            if mul > right {
                break;
            }
            if Self::check_pal(mul) {
                cnt += 1;
            }
        }
        i = 1;
        loop {
            let pal = Self::make_pal_even(i);
            let mul = pal * pal;
            i += 1;
            if mul < left {
                continue;
            }
            if mul > right {
                break;
            }
            if Self::check_pal(mul) {
                cnt += 1;
            }
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_superpalindromes_in_range() {
        assert_eq!(
            Solution::superpalindromes_in_range("4".to_string(), "1000".to_string()),
            4
        );
        assert_eq!(Solution::superpalindromes_in_range("1".to_string(), "2".to_string()), 1);
    }
}
