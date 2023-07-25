// https://leetcode.com/problems/largest-palindrome-product/
// 479. Largest Palindrome Product
pub struct Solution;
impl Solution {
    fn check_palindrome(mut n: i64, mut n10: i64) -> bool {
        while n != 0 {
            if n % 10 != n / n10 {
                return false;
            }
            n -= n / n10 * n10;
            n /= 10;
            n10 /= 100;
        }
        true
    }
    pub fn largest_palindrome(n: i32) -> i32 {
        if n == 1 {
            return 9;
        }
        let mut q = std::collections::BinaryHeap::new();
        let mut n10 = 1i64;
        for _ in 0..n {
            n10 *= 10;
        }
        let n9 = n10 - 1;
        n10 = n10 * n10 / 10;
        if n9 % 11 == 0 {
            q.push((n9 * n9, n9, n9));
        } else {
            let n11 = n9 / 11 * 11;
            q.push((n9 * n11, n9, n11));
        }
        while let Some((s, a, b)) = q.pop() {
            if Self::check_palindrome(s, n10) {
                return (s % 1337) as i32;
            }
            if b % 11 == 0 && a > b && (a - 1 == b || (a - 1) % 11 != 0) {
                q.push(((a - 1) * b, a - 1, b));
                if a == n9 && a % 11 != 0 {
                    q.push((a * (b - 11), a, b - 11))
                }
            }
            if a % 11 == 0 {
                q.push((a * (b - 1), a, b - 1));
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn largest_palindrome() {
        // assert_eq!(Solution::largest_palindrome(8), 475);
        assert_eq!(Solution::largest_palindrome(3), 123);
        assert_eq!(Solution::largest_palindrome(2), 987);
        assert_eq!(Solution::largest_palindrome(1), 9);
    }
}
