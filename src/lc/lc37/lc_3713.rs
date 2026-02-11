// https://leetcode.com/problems/longest-balanced-substring-i/
// 3713. Longest Balanced Substring I
pub struct Solution;
impl Solution {
    pub fn longest_balanced(s: String) -> i32 {
        let mut ans = 0;
        let s = s.as_bytes();
        for i in 0..s.len() {
            let mut cnt = [0; 26];
            let mut mc = 0;
            let mut k = 0;
            for j in i..s.len() {
                let ic = (s[j] - b'a') as usize;
                cnt[ic] += 1;
                if cnt[ic] > mc {
                    mc = cnt[ic];
                }
                if cnt[ic] == 1 {
                    k += 1;
                }
                let l = (j - i + 1) as i32;
                if k * mc == l {
                    ans = ans.max(l);
                }
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn longest_balanced() {
        assert_eq!(Solution::longest_balanced("abbac".to_string()), 4);
        assert_eq!(Solution::longest_balanced("zzabccy".to_string()), 4);
        assert_eq!(Solution::longest_balanced("aba".to_string()), 2);
    }
}
