// https://leetcode.com/problems/subsequence-with-the-minimum-score/
// 2565. Subsequence With the Minimum Score
pub struct Solution;
impl Solution {
    fn check(prefix: &Vec<usize>, suffix: &Vec<usize>, l: usize) -> bool {
        if prefix[prefix.len() - l - 1] != usize::MAX || suffix[l] != usize::MAX {
            return true;
        }
        for i in 0..prefix.len() - l - 1 {
            if prefix[i] == usize::MAX {
                return false;
            }
            if suffix[l + i + 1] != usize::MAX && suffix[l + i + 1] > prefix[i] {
                return true;
            }
        }
        false
    }
    pub fn minimum_score(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut prefix = vec![usize::MAX; t.len()];
        let mut suffix = vec![usize::MAX; t.len()];
        let mut i = 0;
        let mut j = 0;
        while i < s.len() && j < t.len() {
            if s[i] == t[j] {
                prefix[j] = i;
                i += 1;
                j += 1;
            } else {
                i += 1;
            }
        }
        if prefix[t.len() - 1] != usize::MAX {
            return 0;
        }
        i = s.len() - 1;
        j = t.len() - 1;
        loop {
            if s[i] == t[j] {
                suffix[j] = i;
                if i == 0 || j == 0 {
                    break;
                }
                i -= 1;
                j -= 1;
            } else {
                if i == 0 {
                    break;
                }
                i -= 1;
            }
        }
        let mut low = 0;
        let mut high = t.len();
        while low + 1 < high {
            let mid = (low + high) / 2;
            if Self::check(&prefix, &suffix, mid) {
                high = mid;
            } else {
                low = mid;
            }
        }
        high as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_score() {
        assert_eq!(Solution::minimum_score("ba".to_string(), "cb".to_string()), 1);
        assert_eq!(Solution::minimum_score("abacaba".to_string(), "bzaa".to_string()), 1);
        assert_eq!(Solution::minimum_score("cde".to_string(), "xyz".to_string()), 3);
    }
}
