// https://leetcode.com/problems/find-the-difference/
// 389. Find the Difference
pub struct Solution;
impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut m = std::collections::HashMap::<char, i32>::new();
        s.chars().for_each(|c| *m.entry(c).or_default() += 1);
        for c in t.chars() {
            if let Some(cnt) = m.get_mut(&c) {
                if *cnt == 0 {
                    return c;
                }
                *cnt -= 1;
            } else {
                return c;
            }
        }
        ' '
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_the_difference() {
        assert_eq!(
            Solution::find_the_difference("abcd".to_string(), "abcde".to_string()),
            'e'
        );
        assert_eq!(
            Solution::find_the_difference("".to_string(), "y".to_string()),
            'y'
        );
    }
}
