// https://leetcode.com/problems/make-the-string-great/
// 1544. Make The String Great
pub struct Solution;
impl Solution {
    pub fn make_good(s: String) -> String {
        let mut v = vec![];
        for c in s.chars() {
            if let Some(&last) = v.last() {
                if c != last && c.to_lowercase().next().unwrap() == last.to_lowercase().next().unwrap() {
                    v.pop();
                } else {
                    v.push(c);
                }
            } else {
                v.push(c);
            }
        }
        v.into_iter().collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_make_good() {
        assert_eq!(Solution::make_good("leEeetcode".to_string()), "leetcode");
        assert_eq!(Solution::make_good("abBAcC".to_string()), "");
        assert_eq!(Solution::make_good("s".to_string()), "s");
    }
}
