// https://leetcode.com/problems/count-odd-numbers-in-an-interval-range/
// 1523. Count Odd Numbers in an Interval Range
pub struct Solution;
impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        let low = low | 1;
        let high = high + high % 2 - 1;
        (high - low) / 2 + 1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_odds() {
        assert_eq!(Solution::count_odds(3, 7), 3);
        assert_eq!(Solution::count_odds(8, 10), 1);
    }
}
