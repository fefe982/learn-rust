// https://leetcode.com/problems/minimum-substring-partition-of-equal-character-frequency/
// 3144. Minimum Substring Partition of Equal Character Frequencies
pub struct Solution;
impl Solution {
    fn count(s: &[u8], cache: &mut Vec<i32>) -> i32 {
        if s.len() == 0 {
            return 0;
        }
        if cache[s.len()] != 0 {
            return cache[s.len()];
        }
        let mut cnt = vec![0; 26];
        let mut n = 0;
        let mut max_cnt = 0;
        let mut min_cnt = i32::MAX;
        for (i, &c) in s.iter().enumerate() {
            let idx = (c - b'a') as usize;
            cnt[idx] += 1;
            if cnt[idx] == 1 {
                n += 1;
            }
            max_cnt = max_cnt.max(cnt[idx]);
            if max_cnt * n == i as i32 + 1 {
                min_cnt = min_cnt.min(Self::count(&s[i + 1..], cache) + 1);
            }
        }
        min_cnt
    }
    pub fn minimum_substrings_in_partition(s: String) -> i32 {
        let s = s.as_bytes();
        Self::count(s, &mut vec![0; s.len() + 1])
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_string_in_partition() {
        assert_eq!(Solution::minimum_substrings_in_partition("fabccddg".to_string()), 3);
        assert_eq!(Solution::minimum_substrings_in_partition("abababaccddb".to_string()), 2);
    }
}
