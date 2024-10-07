// https://leetcode.com/problems/minimum-string-length-after-removing-substrings/
// 2696. Minimum String Length After Removing Substrings
pub struct Solution;
impl Solution {
    pub fn min_length(s: String) -> i32 {
        let mut v = vec![];
        for c in s.chars() {
            let last = v.last().cloned();
            match (last, c) {
                (Some('A'), 'B') | (Some('C'), 'D') => {
                    v.pop();
                }
                _ => {
                    v.push(c);
                }
            }
        }
        v.len() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_length() {
        assert_eq!(Solution::min_length("ABFCACDB".to_string()), 2);
        assert_eq!(Solution::min_length("ACBBD".to_string()), 5);
    }
}
