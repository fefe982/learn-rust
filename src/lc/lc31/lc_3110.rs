// https://leetcode.com/problems/score-of-a-string/
// 3110. Score of a String
pub struct Solution;
impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        let s = s.as_bytes();
        s.iter()
            .fold((0, s[0]), |(acc, last), &c| (acc + (last as i32 - c as i32).abs(), c))
            .0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_score_of_string() {
        assert_eq!(Solution::score_of_string(String::from("hello")), 13);
        assert_eq!(Solution::score_of_string(String::from("zaz")), 50);
    }
}
