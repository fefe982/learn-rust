// https://leetcode.com/problems/minimum-remove-to-make-valid-parentheses/
// 1249. Minimum Remove to Make Valid Parentheses
pub struct Solution;
impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        s.chars()
            .rev()
            .scan(0, |cnt, ch| {
                if ch == ')' {
                    *cnt += 1;
                } else if ch == '(' {
                    if *cnt > 0 {
                        *cnt -= 1;
                    } else {
                        return Some((-1, ch));
                    }
                }
                Some((*cnt, ch))
            })
            .filter_map(|(c, ch)| if c >= 0 { Some(ch) } else { None })
            .collect::<Vec<char>>()
            .into_iter()
            .rev()
            .scan(0, |cnt, ch| {
                if ch == '(' {
                    *cnt += 1;
                } else if ch == ')' {
                    if *cnt > 0 {
                        *cnt -= 1;
                    } else {
                        return Some((-1, ch));
                    }
                }
                Some((*cnt, ch))
            })
            .filter_map(|(c, ch)| if c >= 0 { Some(ch) } else { None })
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_remove_to_make_valid() {
        assert_eq!(
            Solution::min_remove_to_make_valid("lee(t(c)o)de)".to_string()),
            "lee(t(c)o)de"
        );
        assert_eq!(Solution::min_remove_to_make_valid("a)b(c)d".to_string()), "ab(c)d");
        assert_eq!(Solution::min_remove_to_make_valid("))((".to_string()), "".to_string());
    }
}
