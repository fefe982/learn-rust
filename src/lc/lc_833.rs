// https://leetcode.com/problems/find-and-replace-in-string/
// 833. Find And Replace in String
pub struct Solution;
impl Solution {
    pub fn find_replace_string(
        s: String,
        indices: Vec<i32>,
        sources: Vec<String>,
        targets: Vec<String>,
    ) -> String {
        let mut ist: Vec<(usize, &[u8], &[u8])> = indices
            .into_iter()
            .zip(sources.iter().map(|s| s.as_bytes()))
            .zip(targets.iter().map(|s| s.as_bytes()))
            .map(|((i, s), t)| (i as usize, s, t))
            .collect();
        ist.sort_unstable();
        let src = s.as_bytes();
        let mut tgt = vec![];
        let mut idx = 0;
        for (i, s, t) in ist {
            if i + s.len() <= src.len() && &src[i..i + s.len()] == s {
                if i > idx {
                    tgt.extend_from_slice(&src[idx..i]);
                }
                tgt.extend_from_slice(t);
                idx = i + s.len();
            }
        }
        if idx < src.len() {
            tgt.extend_from_slice(&src[idx..])
        }
        String::from_utf8(tgt).unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn find_replace_string() {
        assert_eq!(
            Solution::find_replace_string(
                "abcde".to_string(),
                vec![2, 2],
                vec_str!["cdef", "bc"],
                vec_str!["f", "fe"]
            ),
            "abcde".to_string()
        );
        assert_eq!(
            Solution::find_replace_string(
                "abcd".to_string(),
                vec![0, 2],
                vec_str!["a", "cd"],
                vec_str!["eee", "ffff"]
            ),
            "eeebffff".to_string()
        );
        assert_eq!(
            Solution::find_replace_string(
                "abcd".to_string(),
                vec![0, 2],
                vec_str!["ab", "ec"],
                vec_str!["eee", "ffff"]
            ),
            "eeecd".to_string()
        );
    }
}
