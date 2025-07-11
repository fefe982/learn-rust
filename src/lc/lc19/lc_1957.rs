// https://leetcode.com/problems/delete-characters-to-make-fancy-string/
// 1957. Delete Characters to Make Fancy String
pub struct Solution;
impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut res = String::new();
        let mut last = ' ';
        let mut cnt = 0;
        for c in s.chars() {
            if c == last {
                if cnt < 2 {
                    res.push(c);
                    cnt += 1;
                }
            } else {
                res.push(c);
                last = c;
                cnt = 1;
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_make_fancy_string() {
        assert_eq!(
            Solution::make_fancy_string("leeetcode".to_string()),
            "leetcode".to_string()
        );
        assert_eq!(Solution::make_fancy_string("aaabaaaa".to_string()), "aabaa".to_string());
        assert_eq!(Solution::make_fancy_string("aab".to_string()), "aab".to_string());
    }
}
