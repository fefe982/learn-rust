// https://leetcode.com/problems/distinct-subsequences-ii/
// 940. Distinct Subsequences II
pub struct Solution;
impl Solution {
    pub fn distinct_subseq_ii(s: String) -> i32 {
        let modulo = 1_0000_0000_7i64;
        let s = s.as_bytes();
        let mut last = vec![usize::MAX; 26];
        let mut cnt = vec![0i64; s.len() + 1];
        cnt[0] = 1;
        for i in 0..s.len() {
            let c = s[i] - b'a';
            let prev = last[c as usize];
            if prev == usize::MAX {
                cnt[i + 1] = cnt[i] * 2 % modulo;
            } else {
                cnt[i + 1] = (cnt[i] * 2 - cnt[prev] + modulo) % modulo;
            }
            last[c as usize] = i;
        }
        ((cnt[s.len()] + modulo - 1) % modulo) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_distinct_subseq_ii() {
        assert_eq!(Solution::distinct_subseq_ii(String::from("zzc")), 5);
        assert_eq!(Solution::distinct_subseq_ii(String::from("abc")), 7);
        assert_eq!(Solution::distinct_subseq_ii(String::from("aba")), 6);
        assert_eq!(Solution::distinct_subseq_ii(String::from("aaa")), 3);
    }
}
