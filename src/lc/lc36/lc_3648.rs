// https://leetcode.com/problems/minimum-sensors-to-cover-grid/
// 3648. Minimum Sensors to Cover Grid
pub struct Solution;
impl Solution {
    pub fn min_sensors(n: i32, m: i32, k: i32) -> i32 {
        let kk = k * 2 + 1;
        (n + kk - 1) / kk * ((m + kk - 1) / kk)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_sensors() {
        assert_eq!(Solution::min_sensors(5, 5, 1), 4);
        assert_eq!(Solution::min_sensors(2, 2, 2), 1);
    }
}
