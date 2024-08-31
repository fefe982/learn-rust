// https://leetcode.com/problems/check-if-point-is-reachable/
// 2543. Check if Point Is Reachable
pub struct Solution;
impl Solution {
    pub fn is_reachable(target_x: i32, target_y: i32) -> bool {
        let mut a = target_x;
        let mut b = target_y;
        let gcd = loop {
            if a == 0 {
                break b;
            }
            b = b % a;
            if b == 0 {
                break a;
            }
            a = a % b;
        };
        if gcd & (gcd - 1) == 0 {
            true
        } else {
            false
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_reachable() {
        assert_eq!(Solution::is_reachable(6, 9), false);
        assert_eq!(Solution::is_reachable(4, 7), true);
    }
}
