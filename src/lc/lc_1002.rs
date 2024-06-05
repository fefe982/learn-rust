// https://leetcode.com/problems/find-common-characters/
// 1002. Find Common Characters
pub struct Solution;
impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        words
            .into_iter()
            .fold(vec![i32::MAX; 26], |cnt, word| {
                let mut ncnt = vec![0; 26];
                for c in word.bytes() {
                    let ic = (c - b'a') as usize;
                    ncnt[ic] = (ncnt[ic] + 1).min(cnt[ic]);
                }
                ncnt
            })
            .into_iter()
            .enumerate()
            .map(|(i, cnt)| std::iter::repeat(((i as u8 + b'a') as char).to_string()).take(cnt as usize))
            .flatten()
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_common_chars() {
        assert_eq!(
            Solution::common_chars(vec_str!["bella", "label", "roller"]),
            vec_str!["e", "l", "l"]
        );
        assert_eq!(
            Solution::common_chars(vec_str!["cool", "lock", "cook"]),
            vec_str!["c", "o"]
        );
    }
}
