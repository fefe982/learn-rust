// https://leetcode.com/problems/longest-substring-with-at-least-k-repeating-characters/
// 395. Longest Substring with At Least K Repeating Characters
pub struct Solution;
impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        if k == 1 {
            return s.len() as i32;
        }
        let mut res = 0;
        for uniq in 1..=26 {
            let mut c = [0; 26];
            let mut cu = 0;
            let mut i = 0;
            let mut cless = 0;
            for j in 0..s.len() {
                let idx = (s[j] - b'a') as usize;
                if c[idx] == 0 {
                    cu += 1;
                    c[idx] = 1;
                    cless += 1;
                    while cu > uniq {
                        let idxi = (s[i] - b'a') as usize;
                        if c[idxi] == k {
                            cless += 1;
                        }
                        c[idxi] -= 1;
                        if c[idxi] == 0 {
                            cu -= 1;
                            cless -= 1;
                        }
                        i += 1;
                    }
                } else {
                    c[idx] += 1;
                    if c[idx] == k {
                        cless -= 1;
                    }
                    if cless == 0 {
                        res = res.max(j - i + 1);
                    }
                }
            }
            if i == 0 {
                break;
            }
        }
        res as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn longest_substring() {
        assert_eq!(Solution::longest_substring("aaabb".to_string(), 3), 3);
        assert_eq!(Solution::longest_substring("ababbc".to_string(), 2), 5);
    }
}
