// https://leetcode.com/problems/angle-between-hands-of-a-clock/
// 1344. Angle Between Hands of a Clock
pub struct Solution;
impl Solution {
    pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
        let h = (hour % 12) as f64 * 30.0 + (minutes as f64 / 60.0) * 30.0;
        let m = (minutes as f64 / 60.0) * 360.0;
        let diff = (h - m).abs();
        diff.min(360.0 - diff)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    #[test]
    fn angle_clock() {
        assert_approx_eq!(Solution::angle_clock(12, 30), 165.0, 1e-5);
        assert_approx_eq!(Solution::angle_clock(3, 30), 75.0, 1e-5);
        assert_approx_eq!(Solution::angle_clock(3, 15), 7.5, 1e-5);
    }
}
