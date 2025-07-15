// https://leetcode.com/problems/lexicographically-smallest-string-after-a-swap/
// 3216. Lexicographically Smallest String After a Swap
pub struct Solution;
impl Solution {
    pub fn get_smallest_string(s: String) -> String {
        let mut res = String::new();
        let mut last = ' ';
        for c in s.chars() {
            if last == ' ' {
                last = c;
            } else if last == '.' {
                res.push(c);
            } else if c < last && last as u8 % 2 == c as u8 % 2 {
                res.push(c);
                res.push(last);
                last = '.';
            } else {
                res.push(last);
                last = c
            }
        }
        if last != ' ' && last != '.' {
            res.push(last);
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_smallest_string() {
        assert_eq!(Solution::get_smallest_string("45320".to_string()), "43520".to_string());
        assert_eq!(Solution::get_smallest_string("001".to_string()), "001".to_string());
    }
}
