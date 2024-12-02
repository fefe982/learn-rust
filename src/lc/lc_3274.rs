// https://leetcode.com/problems/check-if-two-chessboard-squares-have-the-same-color/
// 3274. Check if Two Chessboard Squares Have the Same Color
pub struct Solution;
impl Solution {
    pub fn check_two_chessboards(coordinate1: String, coordinate2: String) -> bool {
        let coordinate1 = coordinate1.as_bytes();
        let coordinate2 = coordinate2.as_bytes();
        (coordinate1[0] - coordinate1[1]) % 2 == (coordinate2[0] - coordinate2[1]) % 2
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check_two_chessboards() {
        assert_eq!(
            Solution::check_two_chessboards("a1".to_string(), "c3".to_string()),
            true
        );
        assert_eq!(
            Solution::check_two_chessboards("a1".to_string(), "h3".to_string()),
            false
        );
    }
}
