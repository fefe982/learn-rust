// https://leetcode.com/problems/occurrences-after-bigram/
// 1078. Occurrences After Bigram
pub struct Solution;
impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        let mut c = 0;
        let mut res = Vec::new();
        for w in text.split_whitespace() {
            if c == 2 {
                res.push(w.to_string());
                if w == first {
                    if w == second {
                        c = 2;
                    } else {
                        c = 1;
                    }
                } else {
                    c = 0;
                }
            } else if c == 1 {
                if w == second {
                    c = 2;
                } else if w == first {
                    c = 1;
                } else {
                    c = 0;
                }
            } else {
                if w == first {
                    c = 1;
                } else {
                    c = 0;
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
    fn find_ocurrences() {
        assert_eq!(
            Solution::find_ocurrences(
                "alice is a good girl she is a good student".to_string(),
                "a".to_string(),
                "good".to_string()
            ),
            vec_str!["girl", "student"]
        );
        assert_eq!(
            Solution::find_ocurrences(
                "we will we will rock you".to_string(),
                "we".to_string(),
                "will".to_string()
            ),
            vec_str!["we", "rock"]
        );
    }
}
