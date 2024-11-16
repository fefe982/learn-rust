// https://leetcode.com/problems/alternating-groups-i/
// 3206. Alternating Groups I
pub struct Solution;
impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>) -> i32 {
        super::lc_3208::Solution::number_of_alternating_groups(colors, 3)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn number_of_alternating_groups() {
        assert_eq!(Solution::number_of_alternating_groups(vec![1, 1, 1]), 0);
        assert_eq!(Solution::number_of_alternating_groups(vec![0, 1, 0, 0, 1]), 3);
    }
}
