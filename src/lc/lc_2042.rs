// https://leetcode.com/problems/check-if-numbers-are-ascending-in-a-sentence/
// 2042. Check if Numbers Are Ascending in a Sentence
pub struct Solution;

impl Solution {
    pub fn are_numbers_ascending(s: String) -> bool {
        let mut last = -1;
        for part in s.split(' ') {
            if let Ok(num) = part.parse::<i32>() {
                if num <= last {
                    return false;
                }
                last = num;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn are_numbers_ascending() {
        assert_eq!(
            Solution::are_numbers_ascending(
                "1 box has 3 blue 4 red 6 green and 12 yellow marbles".to_string()
            ),
            true
        );
        assert_eq!(
            Solution::are_numbers_ascending("hello world 5 x 5".to_string()),
            false
        );
        assert_eq!(
            Solution::are_numbers_ascending(
                "sunset is at 7 51 pm overnight lows will be in the low 50 and 60 s".to_string()
            ),
            false
        )
    }
}
