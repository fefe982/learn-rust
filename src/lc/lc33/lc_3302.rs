// https://leetcode.com/problems/find-the-lexicographically-smallest-valid-sequence/
// 3302. Find the Lexicographically Smallest Valid Sequence
pub struct Solution;
impl Solution {
    pub fn valid_sequence(word1: String, word2: String) -> Vec<i32> {
        let word1 = word1.as_bytes();
        let word2 = word2.as_bytes();
        let n = word1.len();
        let m = word2.len();
        let mut suf = vec![0; n + 1];
        suf[n] = m;
        let mut j = m;
        for i in (0..n).rev() {
            if j > 0 && word1[i] == word2[j - 1] {
                j -= 1;
            }
            suf[i] = j;
        }
        let mut changed = false;
        j = 0;
        let mut ans = Vec::with_capacity(n);
        for (i, &c) in word1.iter().enumerate() {
            if c == word2[j] || (!changed && suf[i + 1] <= j + 1) {
                if c != word2[j] {
                    changed = true;
                }
                ans.push(i as i32);
                j += 1;
                if j == m {
                    return ans;
                }
            }
        }
        vec![]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn valid_sequence() {
        assert_eq!(
            Solution::valid_sequence("vbcca".to_string(), "abc".to_string()),
            vec![0, 1, 2]
        );
        assert_eq!(
            Solution::valid_sequence("bacdc".to_string(), "abc".to_string()),
            vec![1, 2, 4]
        );
        assert_eq!(
            Solution::valid_sequence("aaaaaa".to_string(), "aaabc".to_string()),
            vec![]
        );
        assert_eq!(
            Solution::valid_sequence("abc".to_string(), "ab".to_string()),
            vec![0, 1]
        );
    }
}
