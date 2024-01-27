// https://leetcode.com/problems/water-and-jug-problem/
// 365. Water and Jug Problem
pub struct Solution;
impl Solution {
    pub fn can_measure_water(jug1_capacity: i32, jug2_capacity: i32, target_capacity: i32) -> bool {
        let mut a = jug1_capacity;
        let mut b = jug2_capacity;
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
        target_capacity % gcd == 0 && target_capacity <= jug1_capacity + jug2_capacity
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
