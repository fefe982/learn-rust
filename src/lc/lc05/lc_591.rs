// https://leetcode.com/problems/tag-validator/
// 591. Tag Validator
pub struct Solution;
impl Solution {
    pub fn is_valid(code: String) -> bool {
        let code = code.as_bytes();
        let mut q = vec![];
        let mut i = 0;
        while i < code.len() {
            if code[i] == b'<' {
                if i + 1 >= code.len() {
                    return false;
                }
                if code[i + 1] == b'!' {
                    if q.is_empty() {
                        return false;
                    }
                    if i + 9 > code.len() || &code[i..i + 9] != b"<![CDATA[" {
                        return false;
                    }
                    for j in i + 9..code.len() - 1 {
                        if j + 2 == code.len() {
                            return false;
                        }
                        if &code[j..j + 3] == b"]]>" {
                            i = j + 2;
                            break;
                        }
                    }
                } else {
                    let mut s = i + 1;
                    let mut close = false;
                    if code[i + 1] == b'/' {
                        close = true;
                        s += 1;
                    }
                    let mut len = 0;
                    for j in s..code.len() {
                        if code[j] >= b'A' && code[j] <= b'Z' {
                            len += 1;
                            if len > 9 {
                                return false;
                            }
                        } else if code[j] == b'>' {
                            if len == 0 {
                                return false;
                            }
                            if !close {
                                q.push(code[s..j].to_owned());
                            } else {
                                if let Some(n) = q.pop() {
                                    if &n[..] != &code[s..j] {
                                        return false;
                                    }
                                } else {
                                    return false;
                                }
                            }
                            i = j;
                            break;
                        } else {
                            return false;
                        }
                    }
                }
                if q.is_empty() && i + 1 != code.len() {
                    return false;
                }
            } else {
                if q.is_empty() {
                    return false;
                }
            }
            i += 1;
        }
        q.is_empty()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_valid() {
        assert_eq!(Solution::is_valid("<A><!A></A>".to_string()), false);
        assert_eq!(Solution::is_valid("<A><![CDATA[</A>]]123></A>".to_string()), false);
        assert_eq!(Solution::is_valid("<A></A><B></B>".to_string()), false);
        assert_eq!(
            Solution::is_valid("<![CDATA[wahaha]]]><![CDATA[]> wahaha]]>".to_string()),
            false
        );
        assert_eq!(
            Solution::is_valid("<DIV>This is the first line <![CDATA[<div>]]></DIV>".to_string()),
            true
        );
        assert_eq!(
            Solution::is_valid("<DIV>>>  ![cdata[]] <![CDATA[<div>]>]]>]]>>]</DIV>".to_string()),
            true
        );
        assert_eq!(Solution::is_valid("<A>  <B> </A>   </B>".to_string()), false);
    }
}
