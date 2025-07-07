// https://leetcode.com/problems/find-the-highest-altitude/
// 1732. Find the Highest Altitude
pub struct Solution;
impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        gain.into_iter()
            .fold((0, 0), |(m, a), g| (std::cmp::max(m, a + g), a + g))
            .0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn largest_altitude() {
        assert_eq!(Solution::largest_altitude(vec![-5, 1, 5, 0, -7]), 1);
        assert_eq!(Solution::largest_altitude(vec![-4, -3, -2, -1, 4, 3, 2]), 0);
    }
}
