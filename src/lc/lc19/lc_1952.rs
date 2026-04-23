// https://leetcode.com/problems/three-divisors/
// 1952. Three Divisors
pub struct Solution;
impl Solution {
    pub fn is_three(n: i32) -> bool {
        if n < 2 {
            return false;
        }
        for i in 2.. {
            if n % i == 0 {
                return n / i == i;
            }
            if i * i > n {
                return false;
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_three() {
        assert_eq!(Solution::is_three(2), false);
        assert_eq!(Solution::is_three(4), true);
    }
}
