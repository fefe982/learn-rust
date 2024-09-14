// https://leetcode.com/problems/removing-stars-from-a-string/
// 2390. Removing Stars From a String
pub struct Solution;
impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut v = "".to_string();
        for c in s.chars() {
            if c != '*' {
                v.push(c);
            } else {
                v.pop();
            }
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn remove_stars() {
        assert_eq!(
            Solution::remove_stars(String::from("leet**cod*e")),
            String::from("lecoe")
        );
        assert_eq!(Solution::remove_stars(String::from("erase*****")), String::from(""));
    }
}
