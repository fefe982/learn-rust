// https://leetcode.com/problems/count-substrings-with-k-frequency-characters-i/
// 3325. Count Substrings With K Frequency Characters
pub struct Solution;
impl Solution {
    pub fn number_of_substrings(s: String, k: i32) -> i32 {
        let mut cnt = [0; 26];
        let mut sum = 0;
        let mut i = 0;
        let s = s.as_bytes();
        for j in 0..s.len() {
            let jc = (s[j] - b'a') as usize;
            cnt[jc] += 1;
            while cnt[jc] >= k {
                let ic = (s[i] - b'a') as usize;
                cnt[ic] -= 1;
                i += 1;
            }
            sum += i as i32;
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_number_of_substrings() {
        assert_eq!(Solution::number_of_substrings("ajsrhoebe".to_string(), 2), 7);
        assert_eq!(Solution::number_of_substrings("abacb".to_string(), 2), 4);
        assert_eq!(Solution::number_of_substrings("abcde".to_string(), 1), 15);
    }
}
