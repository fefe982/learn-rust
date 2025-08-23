// https://leetcode.com/problems/remove-all-adjacent-duplicates-in-string/
// 1047. Remove All Adjacent Duplicates In String
pub struct Solution;
impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut res = String::new();
        for c in s.chars() {
            if res.is_empty() || res.chars().last().unwrap() != c {
                res.push(c);
            } else {
                res.pop();
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn remove_duplicates() {
        assert_eq!(Solution::remove_duplicates("abbaca".to_string()), "ca");
        assert_eq!(Solution::remove_duplicates("azxxzy".to_string()), "ay");
    }
}
