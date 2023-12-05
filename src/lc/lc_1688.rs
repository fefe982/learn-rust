// https://leetcode.com/problems/count-of-matches-in-tournament/
// 1688. Count of Matches in Tournament
pub struct Solution;
impl Solution {
    pub fn number_of_matches(n: i32) -> i32 {
        n - 1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_number_of_matches() {
        assert_eq!(Solution::number_of_matches(7), 6);
        assert_eq!(Solution::number_of_matches(14), 13);
    }
}
