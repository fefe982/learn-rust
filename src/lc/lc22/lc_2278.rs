// https://leetcode.com/problems/percentage-of-letter-in-string/
// 2278. Percentage of Letter in String
pub struct Solution;
impl Solution {
    pub fn percentage_letter(s: String, letter: char) -> i32 {
        let mut cnt = 0;
        let mut total = 0;
        for c in s.chars() {
            if c == letter {
                cnt += 1;
            }
            total += 1;
        }
        cnt * 100 / total
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_percentage_letter() {
        assert_eq!(Solution::percentage_letter("foobar".to_string(), 'o'), 33);
        assert_eq!(Solution::percentage_letter("jjjj".to_string(), 'k'), 0);
    }
}
