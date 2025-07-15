// https://leetcode.com/problems/prime-palindrome/
// 866. Prime Palindrome
pub struct Solution;
impl Solution {
    pub fn prime_palindrome(n: i32) -> i32 {
        if n > 7 && n <= 11 {
            return 11;
        }
        fn is_prime(n: i32) -> bool {
            if n < 2 {
                return false;
            }
            let mut i = 2;
            while i * i <= n {
                if n % i == 0 {
                    return false;
                }
                i += 1;
            }
            true
        }
        fn check(hn: i32) -> i32 {
            let v = hn.to_string().chars().collect::<Vec<char>>();
            let vv = v
                .iter()
                .chain(v.iter().rev().skip(1))
                .collect::<String>()
                .parse::<i32>()
                .unwrap();
            if is_prime(vv) {
                return vv;
            }
            return 0;
        }
        let ns = n.to_string();
        let v = ns.chars().collect::<Vec<char>>();
        let mut hn = v[0..v.len() / 2 + 1].iter().collect::<String>().parse::<i32>().unwrap();
        if v.len() % 2 == 1 {
            let cn = check(hn);
            if cn >= n {
                return cn;
            }
        } else {
            let mut i = 1;
            while hn / i >= 10 {
                i *= 10;
            }
            hn = i;
            let cn = check(hn);
            if cn >= n {
                return cn;
            }
        }
        loop {
            hn += 1;
            let cn = check(hn);
            if cn >= n {
                return cn;
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn prime_palindrome() {
        assert_eq!(Solution::prime_palindrome(2), 2);
        assert_eq!(Solution::prime_palindrome(9989900), 100030001);
        assert_eq!(Solution::prime_palindrome(6), 7);
        assert_eq!(Solution::prime_palindrome(8), 11);
        assert_eq!(Solution::prime_palindrome(13), 101);
    }
}
