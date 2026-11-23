// https://leetcode.com/problems/complete-prime-number/
// 3765. Complete Prime Number
pub struct Solution;
impl Solution {
    pub fn complete_prime(num: i32) -> bool {
        let mut num = num;
        let is_prime = |n: i32| {
            if n < 2 {
                return false;
            }
            for i in 2..=((n as f64).sqrt() as i32) {
                if n % i == 0 {
                    return false;
                }
            }
            true
        };
        if !is_prime(num) {
            return false;
        }
        let mut suf = 0;
        let mut pow = 1;
        while num > 0 {
            let rem = num % 10;
            let pre = num / 10;
            if pre == 0 {
                break;
            }
            if !is_prime(pre) {
                return false;
            }
            num /= 10;
            suf += rem * pow;
            pow *= 10;
            if !is_prime(suf) {
                return false;
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn complete_prime() {
        assert_eq!(Solution::complete_prime(23), true);
        assert_eq!(Solution::complete_prime(39), false);
        assert_eq!(Solution::complete_prime(7), true);
    }
}
