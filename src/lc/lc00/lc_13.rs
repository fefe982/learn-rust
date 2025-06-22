// https://leetcode.com/problems/roman-to-integer/
// 13. Roman to Integer
pub struct Solution;
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let map = std::collections::HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);
        let s = s.chars().map(|c| map[&c]).collect::<Vec<i32>>();
        let mut sum = 0;
        for i in 0..s.len() {
            if i + 1 < s.len() && s[i] < s[i + 1] {
                sum -= s[i];
            } else {
                sum += s[i];
            }
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn roman_to_int() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
