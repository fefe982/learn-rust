// https://leetcode.com/problems/count-asterisks/
// 2315. Count Asterisks
pub struct Solution;
impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        let mut cnt = 0;
        let mut in_bar = false;
        for c in s.chars() {
            if c == '|' {
                in_bar = !in_bar;
            } else if c == '*' && !in_bar {
                cnt += 1;
            }
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_asterisks() {
        assert_eq!(Solution::count_asterisks(String::from("l|*e*et|c**o|*de|")), 2);
        assert_eq!(Solution::count_asterisks(String::from("iamprogrammer")), 0);
        assert_eq!(Solution::count_asterisks(String::from("yo|uar|e**|b|e***au|tifu|l")), 5);
    }
}
