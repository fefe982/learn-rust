// https://leetcode.com/problems/minimum-deletions-to-make-string-balanced/
// 1653. Minimum Deletions to Make String Balanced
pub struct Solution;
impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let mut current_min_count = 0;
        let mut b_count = 0;
        for c in s.chars() {
            if c == 'a' {
                if b_count < current_min_count + 1 {
                    current_min_count = b_count;
                } else {
                    current_min_count = current_min_count + 1;
                }
            } else {
                b_count += 1;
            }
        }
        current_min_count
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_deletions() {
        assert_eq!(Solution::minimum_deletions(String::from("aababbab")), 2);
        assert_eq!(Solution::minimum_deletions(String::from("bbaaaaabb")), 2);
    }
}
