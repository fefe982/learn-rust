// https://leetcode.com/problems/determine-color-of-a-chessboard-square/
// 1812. Determine Color of a Chessboard Square
pub struct Solution;
impl Solution {
    pub fn square_is_white(coordinates: String) -> bool {
        let c = coordinates.as_bytes();
        (c[0] as i32 + c[1] as i32) % 2 == 1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_square_is_white() {
        assert_eq!(Solution::square_is_white("a1".to_string()), false);
        assert_eq!(Solution::square_is_white("h3".to_string()), true);
        assert_eq!(Solution::square_is_white("c7".to_string()), false);
    }
}
