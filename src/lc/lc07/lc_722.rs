// https://leetcode.com/problems/remove-comments/
// 722. Remove Comments
pub struct Solution;
impl Solution {
    pub fn remove_comments(source: Vec<String>) -> Vec<String> {
        let mut in_block = false;
        let mut block_pre = "".to_owned();
        let mut res = vec![];
        'line: for s in source {
            let mut comment_start = usize::MAX;
            let mut block_comment_end = usize::MAX;
            let mut line_start = 0;
            for (i, c) in s.bytes().enumerate() {
                if !in_block {
                    if comment_start != usize::MAX {
                        if c == b'*' {
                            in_block = true;
                            if line_start < comment_start {
                                block_pre += &s[line_start..comment_start];
                            }
                        } else if c == b'/' {
                            block_pre += &s[line_start..comment_start];
                            if !block_pre.is_empty() {
                                res.push(block_pre);
                                block_pre = "".to_owned();
                            }
                            continue 'line;
                        }
                        comment_start = usize::MAX;
                    } else {
                        if c == b'/' {
                            comment_start = i;
                        } else {
                            comment_start = usize::MAX;
                        }
                    }
                } else {
                    if block_comment_end != usize::MAX {
                        if c == b'/' {
                            in_block = false;
                            line_start = i + 1;
                        }
                        block_comment_end = if c == b'*' { i } else { usize::MAX };
                    } else {
                        if c == b'*' {
                            block_comment_end = i;
                        }
                    }
                }
            }
            if !in_block {
                block_pre += &s[line_start..];
                if !block_pre.is_empty() {
                    res.push(block_pre);
                    block_pre = "".to_owned();
                }
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn remove_comments() {
        assert_eq!(
            Solution::remove_comments(vec_str![
                "struct Node{",
                "    /*/ declare members;/**/",
                "    int size;",
                "    /**/int val;",
                "};"
            ]),
            vec_str![
                "struct Node{",
                "    ",
                "    int size;",
                "    int val;",
                "};"
            ]
        );
        assert_eq!(
            Solution::remove_comments(vec_str![
                "/*Test program */",
                "int main()",
                "{ ",
                "  // variable declaration ",
                "int a, b, c;",
                "/* This is a test",
                "   multiline  ",
                "   comment for ",
                "   testing */",
                "a = b + c;",
                "}"
            ]),
            vec_str!["int main()", "{ ", "  ", "int a, b, c;", "a = b + c;", "}"]
        );
        assert_eq!(
            Solution::remove_comments(vec_str!["a/*comment", "line", "more_comment*/b"]),
            vec_str!["ab"]
        );
    }
}
