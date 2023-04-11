// https://leetcode.com/problems/removing-stars-from-a-string/
// 2390. Removing Stars From a String
pub struct Solution;
impl Solution {
    pub fn remove_stars(s: String) -> String {
        let s = s.as_bytes();
        let mut v = Vec::with_capacity(s.len());
        for &c in s {
            if c != b'*' {
                v.push(c);
            } else {
                v.pop();
            }
        }
        String::from_utf8(v).unwrap()
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
        assert_eq!(
            Solution::remove_stars(String::from("erase*****")),
            String::from("")
        );
    }
}
