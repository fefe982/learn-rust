// https://leetcode.com/problems/count-substrings-that-can-be-rearranged-to-contain-a-string-ii/
// 3298. Count Substrings That Can Be Rearranged to Contain a String II
pub struct Solution;
impl Solution {
    pub fn valid_substring_count(word1: String, word2: String) -> i64 {
        let mut cnt = vec![0; 26];
        let mut ccnt = 0;
        for &c in word2.as_bytes() {
            cnt[(c - b'a') as usize] += 1;
            if cnt[(c - b'a') as usize] == 1 {
                ccnt += 1;
            }
        }
        let mut i = 0;
        let word1 = word1.as_bytes();
        let mut res = 0;
        for j in 0..word1.len() {
            let jc = (word1[j] - b'a') as usize;
            cnt[jc] -= 1;
            if cnt[jc] == 0 {
                ccnt -= 1;
            }
            if ccnt == 0 {
                loop {
                    let ic = (word1[i] - b'a') as usize;
                    res += (word1.len() - j) as i64;
                    cnt[ic] += 1;
                    i += 1;
                    if cnt[ic] == 1 {
                        ccnt += 1;
                        break;
                    }
                }
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_valid_substring_count() {
        assert_eq!(
            Solution::valid_substring_count("bcca".to_string(), "bca".to_string()),
            1
        );
        assert_eq!(
            Solution::valid_substring_count("abcabc".to_string(), "abc".to_string()),
            10
        );
        assert_eq!(
            Solution::valid_substring_count("abcabc".to_string(), "aaabc".to_string()),
            0
        );
    }
}
