// https://leetcode.com/problems/convert-the-temperature/
// 2469. Convert the Temperature
pub struct Solution;
impl Solution {
    pub fn convert_temperature(celsius: f64) -> Vec<f64> {
        vec![celsius + 273.15, celsius * 1.8 + 32.0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    #[test]
    fn convert_temperature() {
        let ans = Solution::convert_temperature(36.5);
        assert_approx_eq!(ans[0], 309.65);
        assert_approx_eq!(ans[1], 97.7);
        let ans = Solution::convert_temperature(122.11);
        assert_approx_eq!(ans[0], 395.26);
        assert_approx_eq!(ans[1], 251.798);
    }
}
