// https://leetcode.com/problems/select-k-disjoint-special-substrings/
// 3458. Select K Disjoint Special Substrings
pub struct Solution;
impl Solution {
    pub fn max_substring_length(s: String, k: i32) -> bool {
        if k == 0 {
            return true;
        }
        let s = s.as_bytes();
        let mut start = [usize::MAX; 26];
        let mut end = [usize::MAX; 26];
        for (i, c) in s.iter().enumerate() {
            let c = (c - b'a') as usize;
            if start[c] == usize::MAX {
                start[c] = i;
            }
            end[c] = i;
        }
        let n = s.len();
        let mut interval = vec![];
        'i: for i in 0..n {
            let ci = (s[i] - b'a') as usize;
            if start[ci] != i {
                continue;
            }
            let mut j = end[ci];
            let mut k = i;
            while k <= j {
                let ck = (s[k] - b'a') as usize;
                j = j.max(end[ck]);
                if start[ck] < i {
                    continue 'i;
                }
                k += 1;
            }
            if i == 0 && j == n - 1 {
                continue;
            }
            interval.push((j, i));
        }
        if interval.len() < k as usize {
            return false;
        }
        if k == 1 {
            return true;
        }
        interval.sort();
        let mut cnt = 1;
        let mut end = interval[0].0;
        for i in 1..interval.len() {
            if interval[i].1 > end {
                cnt += 1;
                end = interval[i].0;
            }
            if cnt == k {
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
    fn max_unique_split() {
        assert_eq!(Solution::max_substring_length("aed".to_string(), 1), true);
        assert_eq!(Solution::max_substring_length("abcdbaefab".to_string(), 2), true);
        assert_eq!(Solution::max_substring_length("cdefdc".to_string(), 3), false);
        assert_eq!(Solution::max_substring_length("abeabe".to_string(), 0), true);
    }
}
