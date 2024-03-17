// https://leetcode.com/problems/maximum-number-of-non-overlapping-substrings/
// 1520. Maximum Number of Non-Overlapping Substrings
pub struct Solution;
impl Solution {
    pub fn max_num_of_substrings(s: String) -> Vec<String> {
        let s = s.chars().collect::<Vec<_>>();
        let mut cs = vec![usize::MAX; 26];
        let mut ce = vec![0; 26];
        for (i, &c) in s.iter().enumerate() {
            let idx = (c as u8 - b'a') as usize;
            cs[idx] = cs[idx].min(i);
            ce[idx] = ce[idx].max(i);
        }
        let mut h = std::collections::BinaryHeap::new();
        for i in 0..26 {
            if cs[i] <= ce[i] {
                let mut ss = cs[i];
                let mut ee = ce[i];
                let mut j = cs[i] + 1;
                while j < ee {
                    ss = ss.min(cs[(s[j] as u8 - b'a') as usize]);
                    ee = ee.max(ce[(s[j] as u8 - b'a') as usize]);
                    j += 1;
                }
                if ss == cs[i] {
                    h.push(std::cmp::Reverse((ee, ss)));
                }
            }
        }
        let mut ans = vec![];
        let mut last = 0;
        while let Some(std::cmp::Reverse((ee, ss))) = h.pop() {
            if ss >= last {
                ans.push(s[ss..=ee].iter().collect::<String>());
                last = ee + 1;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_num_of_substrings() {
        assert_eq!(
            Solution::max_num_of_substrings(String::from("cbadabdb")),
            ["c", "badabdb"]
        );
        assert_eq!(Solution::max_num_of_substrings(String::from("abab")), ["abab"]);
        assert_eq!(
            Solution::max_num_of_substrings(String::from("adefaddaccc")),
            ["e", "f", "ccc"]
        );
        assert_eq!(
            Solution::max_num_of_substrings(String::from("abbaccd")),
            ["bb", "cc", "d"]
        );
    }
}
