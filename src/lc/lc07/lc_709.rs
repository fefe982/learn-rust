// https://leetcode.com/problems/to-lower-case/
// 709. To Lower Case
pub struct Solution;
impl Solution {
    pub fn to_lower_case(s: String) -> String {
        s.to_ascii_lowercase()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn to_lower_case() {
        assert_eq!(Solution::to_lower_case("Hello".to_string()), "hello");
        assert_eq!(Solution::to_lower_case("here".to_string()), "here");
        assert_eq!(Solution::to_lower_case("LOVELY".to_string()), "lovely");
    }
}
