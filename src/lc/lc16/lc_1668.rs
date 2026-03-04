// https://leetcode.com/problems/maximum-repeating-substring/
// 1668. Maximum Repeating Substring
pub struct Solution;
impl Solution {
    pub fn max_repeating(sequence: String, word: String) -> i32 {
        let seq = sequence.as_bytes();
        let word = word.as_bytes();
        if word.len() > seq.len() {
            return 0;
        }
        let mut mc = 0;
        for i in 0..=seq.len() - word.len() {
            let mut s = i;
            let mut c = 0;
            while s + word.len() <= seq.len() {
                if seq[s..s + word.len()] == word[..] {
                    s += word.len();
                    c += 1;
                } else {
                    break;
                }
            }
            mc = mc.max(c);
        }
        mc
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_repeating() {
        assert_eq!(Solution::max_repeating("ababc".to_string(), "ab".to_string()), 2);
        assert_eq!(Solution::max_repeating("ababc".to_string(), "ba".to_string()), 1);
        assert_eq!(Solution::max_repeating("ababc".to_string(), "ac".to_string()), 0);
    }
}
