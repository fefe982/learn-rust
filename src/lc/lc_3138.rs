// https://leetcode.com/problems/minimum-length-of-anagram-concatenation/
// 3138. Minimum Length of an Anagram Concatenation
pub struct Solution;
impl Solution {
    pub fn min_anagram_length(s: String) -> i32 {
        let s = s.as_bytes();
        let len = s.len();
        'nexti: for i in 1..=len / 2 {
            if len % i != 0 {
                continue;
            }
            let mut cnt = [0; 26];
            for j in 0..i {
                cnt[(s[j] - b'a') as usize] += 1;
            }
            for k in 1..len / i {
                let mut cnt1 = [0; 26];
                for j in 0..i {
                    cnt1[(s[k * i + j] - b'a') as usize] += 1;
                }
                if cnt != cnt1 {
                    continue 'nexti;
                }
            }
            return i as i32;
        }
        len as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_anagram_length() {
        assert_eq!(Solution::min_anagram_length("abba".to_string()), 2);
        assert_eq!(Solution::min_anagram_length("cdef".to_string()), 4);
    }
}
