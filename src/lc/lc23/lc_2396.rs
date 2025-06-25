// https://leetcode.com/problems/minimum-string-length-after-removing-substrings/
// 2696. Minimum String Length After Removing Substrings
pub struct Solution;
impl Solution {
    pub fn min_length(s: String) -> i32 {
        let mut q = vec![];
        for c in s.chars() {
            let &last = q.last().unwrap_or(&'Z');
            if (last == 'A' && c == 'B') || (last == 'C' && c == 'D') {
                q.pop();
            } else {
                q.push(c);
            }
        }
        q.len() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_min_length() {
        assert_eq!(Solution::min_length(String::from("ABFCACDB")), 2);
        assert_eq!(Solution::min_length(String::from("ACBBD")), 5);
    }
}
