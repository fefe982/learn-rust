// https://leetcode.com/problems/minimum-deletions-to-make-character-frequencies-unique/
// 1647. Minimum Deletions to Make Character Frequencies Unique
pub struct Solution;
impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        let mut freq = vec![0; 26];
        for c in s.chars() {
            freq[(c as u8 - 'a' as u8) as usize] += 1;
        }
        freq.sort_unstable();
        let mut ans = 0;
        for i in (0..25).rev() {
            if freq[i] >= freq[i + 1] {
                if freq[i + 1] == 0 {
                    ans += freq[i];
                    freq[i] = 0;
                } else {
                    ans += freq[i] - freq[i + 1] + 1;
                    freq[i] = freq[i + 1] - 1;
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
    fn min_deletions() {
        assert_eq!(Solution::min_deletions("aab".to_string()), 0);
        assert_eq!(Solution::min_deletions("aaabbbcc".to_string()), 2);
        assert_eq!(Solution::min_deletions("ceabaacb".to_string()), 2);
    }
}
