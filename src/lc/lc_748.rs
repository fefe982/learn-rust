// https://leetcode.com/problems/shortest-completing-word/
// 748. Shortest Completing Word
pub struct Solution;
impl Solution {
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        let mut cnt = [0; 26];
        let mut lcnt = 0;
        for c in license_plate.chars() {
            let b = c as u8;
            let idx = if b >= b'a' && b <= b'z' {
                (b - b'a') as usize
            } else if b >= b'A' && b <= b'Z' {
                (b - b'A') as usize
            } else {
                usize::MAX
            };
            if idx < 26 {
                cnt[idx] += 1;
                if cnt[idx] == 1 {
                    lcnt += 1;
                }
            }
        }
        let mut res = "".to_string();
        let mut len = usize::MAX;
        for w in words.into_iter() {
            if w.len() >= len {
                continue;
            }
            let mut wcnt = [0; 26];
            let mut left = lcnt;
            for c in w.chars() {
                let idx = (c as u8 - b'a') as usize;
                wcnt[idx] += 1;
                if wcnt[idx] == cnt[idx] {
                    left -= 1;
                }
            }
            if left == 0 {
                res = w;
                len = res.len();
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
    fn shortest_completing_word() {
        assert_eq!(
            Solution::shortest_completing_word("1s3 PSt".to_string(), vec_str!["step", "steps", "stripe", "stepple"]),
            "steps"
        );
        assert_eq!(
            Solution::shortest_completing_word("1s3 456".to_string(), vec_str!["looks", "pest", "stew", "show"]),
            "pest"
        );
    }
}
