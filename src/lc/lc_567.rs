// https://leetcode.com/problems/permutation-in-string/
// 567. Permutation in String
pub struct Solution;
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut nz = 0;
        let mut cnt = vec![0; 26];
        let mut n = 0;
        for c in s1.bytes() {
            cnt[(c - b'a') as usize] += 1;
            if cnt[(c - b'a') as usize] == 1 {
                nz += 1;
            }
            n += 1;
        }
        let s2 = s2.as_bytes();
        for i in 0..s2.len() {
            let cidx = (s2[i] - b'a') as usize;
            cnt[cidx] -= 1;
            if cnt[cidx] == 0 {
                nz -= 1;
            } else if cnt[cidx] == -1 {
                nz += 1;
            }
            if i >= n {
                let cidx = (s2[i - n] - b'a') as usize;
                cnt[cidx] += 1;
                if cnt[cidx] == 1 {
                    nz += 1;
                } else if cnt[cidx] == 0 {
                    nz -= 1;
                }
            }
            if nz == 0 {
                return true;
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check_inclusion() {
        assert_eq!(
            Solution::check_inclusion("ab".to_string(), "eidbaooo".to_string()),
            true
        );
        assert_eq!(
            Solution::check_inclusion("ab".to_string(), "eidboaoo".to_string()),
            false
        );
    }
}
