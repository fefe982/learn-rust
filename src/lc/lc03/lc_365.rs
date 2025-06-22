// https://leetcode.com/problems/water-and-jug-problem/
// 365. Water and Jug Problem
pub struct Solution;
impl Solution {
    pub fn can_measure_water(x: i32, y: i32, target: i32) -> bool {
        let mut a = x;
        let mut b = y;
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
        target % gcd == 0 && target <= x + y
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_can_measure_water() {
        assert_eq!(Solution::can_measure_water(3, 5, 4), true);
        assert_eq!(Solution::can_measure_water(2, 6, 5), false);
        assert_eq!(Solution::can_measure_water(1, 2, 3), true);
    }
}
