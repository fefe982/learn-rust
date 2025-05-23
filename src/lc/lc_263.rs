// https://leetcode.com/problems/ugly-number/
// 263. Ugly Number
pub struct Solution;
impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        let mut n = n;
        if n <= 0 {
            return false;
        }
        while n % 2 == 0 {
            n /= 2;
        }
        while n % 3 == 0 {
            n /= 3;
        }
        while n % 5 == 0 {
            n /= 5;
        }
        n == 1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_ugly() {
        assert_eq!(Solution::is_ugly(6), true);
        assert_eq!(Solution::is_ugly(1), true);
        assert_eq!(Solution::is_ugly(14), false);
    }
}
