// https://leetcode.com/problems/power-of-three/
// 326. Power of Three
pub struct Solution;
impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        let mut n = n;
        while n % 3 == 0 {
            n /= 3;
        }
        n == 1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_power_of_three() {
        assert_eq!(Solution::is_power_of_three(27), true);
        assert_eq!(Solution::is_power_of_three(0), false);
        assert_eq!(Solution::is_power_of_three(-1), false);
    }
}
