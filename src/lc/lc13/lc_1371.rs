// https://leetcode.com/problems/find-the-longest-substring-containing-vowels-in-even-counts/
// 1371. Find the Longest Substring Containing Vowels in Even Counts
pub struct Solution;
impl Solution {
    pub fn find_the_longest_substring(s: String) -> i32 {
        let mut cnt = 0;
        let s = s.as_bytes();
        let mut max = 0;
        let mut mp = std::collections::HashMap::<i32, usize>::new();
        mp.insert(0, 0);
        for j in 0..s.len() {
            if s[j] == b'a' || s[j] == b'e' || s[j] == b'i' || s[j] == b'o' || s[j] == b'u' {
                cnt ^= 1 << (s[j] - b'a');
            }
            if let Some(&s) = mp.get(&cnt) {
                max = max.max(j - s + 1);
            } else {
                mp.insert(cnt, j + 1);
            }
        }
        max as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_the_longest_substring() {
        assert_eq!(
            Solution::find_the_longest_substring("eleetminicoworoep".to_string()),
            13
        );
        assert_eq!(Solution::find_the_longest_substring("leetcodeisgreat".to_string()), 5);
        assert_eq!(Solution::find_the_longest_substring("bcbcbc".to_string()), 6);
    }
}
