// https://leetcode.com/problems/text-justification/
// 68. Text Justification
pub struct Solution;
impl Solution {
    fn make_str(
        words: &Vec<&[u8]>,
        s: usize,
        e: usize,
        width: usize,
        max_width: usize,
        ajust_left: bool,
    ) -> String {
        let mut line = Vec::with_capacity(max_width);
        let nwords = e - s;
        if nwords == 1 || ajust_left {
            for iw in s..(e - 1) {
                line.extend_from_slice(words[iw]);
                line.push(b' ');
            }
            line.extend_from_slice(words[e - 1]);
            line.extend(std::iter::repeat(b' ').take(max_width - line.len()));
        } else {
            let blanks = max_width - width;
            let nsep = nwords - 1;
            let sep_len = blanks / nsep;
            let sep_ext = blanks - sep_len * nsep;
            for idx in 0..sep_ext {
                line.extend_from_slice(words[s + idx]);
                line.extend(std::iter::repeat(b' ').take(sep_len + 2));
            }
            for idx in sep_ext..nsep {
                line.extend_from_slice(words[s + idx]);
                line.extend(std::iter::repeat(b' ').take(sep_len + 1));
            }
            line.extend_from_slice(words[e - 1]);
        }
        String::from_utf8(line).unwrap()
    }
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let words: Vec<&[u8]> = words.iter().map(|x| x.as_bytes()).collect();
        let mut para = Vec::new();
        let max_width = max_width as usize;
        let mut s = 0;
        let mut e = 0;
        let mut len = 0;
        while e < words.len() {
            if len + words[e].len() <= max_width {
                len += words[e].len() + 1;
                e += 1;
            } else {
                para.push(Self::make_str(&words, s, e, len - 1, max_width, false));
                s = e;
                len = 0;
            }
        }
        para.push(Self::make_str(&words, s, e, len, max_width, true));
        para
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_str;
    #[test]
    fn full_justify() {
        assert_eq!(
            Solution::full_justify(
                vec_str![
                    "This",
                    "is",
                    "an",
                    "example",
                    "of",
                    "text",
                    "justification."
                ],
                16
            ),
            vec_str!["This    is    an", "example  of text", "justification.  "]
        );
        assert_eq!(
            Solution::full_justify(
                vec_str!["What", "must", "be", "acknowledgment", "shall", "be"],
                16
            ),
            vec_str!["What   must   be", "acknowledgment  ", "shall be        "]
        );
        assert_eq!(
            Solution::full_justify(
                vec_str![
                    "Science",
                    "is",
                    "what",
                    "we",
                    "understand",
                    "well",
                    "enough",
                    "to",
                    "explain",
                    "to",
                    "a",
                    "computer.",
                    "Art",
                    "is",
                    "everything",
                    "else",
                    "we",
                    "do"
                ],
                20
            ),
            vec_str![
                "Science  is  what we",
                "understand      well",
                "enough to explain to",
                "a  computer.  Art is",
                "everything  else  we",
                "do                  "
            ]
        );
    }
}
