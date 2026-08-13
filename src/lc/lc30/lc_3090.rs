// https://leetcode.com/problems/maximum-length-substring-with-two-occurrences/
// 3090. Maximum Length Substring With Two Occurrences
pub struct Solution;
impl Solution {
    pub fn maximum_length_substring(s: String) -> i32 {
        let mut mlen = 0;
        let mut v = [0; 26];
        let s = s.as_bytes();
        let mut i = 0;
        for j in 0..s.len() {
            let c = (s[j] - b'a') as usize;
            v[c] += 1;
            while v[c] > 2 {
                let ci = (s[i] - b'a') as usize;
                v[ci] -= 1;
                i += 1;
            }
            mlen = mlen.max(j - i + 1);
        }
        mlen as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_length_substring() {
        assert_eq!(Solution::maximum_length_substring("bcbbbcba".to_string()), 4);
        assert_eq!(Solution::maximum_length_substring("aaaa".to_string()), 2);
    }
}
