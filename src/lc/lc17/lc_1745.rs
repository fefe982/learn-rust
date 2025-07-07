// https://leetcode.com/problems/palindrome-partitioning-iv/
// 1745. Palindrome Partitioning IV
pub struct Solution;
impl Solution {
    pub fn check_partitioning(s: String) -> bool {
        let s = s.chars().collect::<Vec<_>>();
        let mut pal = vec![vec![false; s.len()]; s.len()];
        for i in 0..s.len() {
            pal[i][i] = true;
            for j in (0..i).rev() {
                if s[i] == s[j] && (i - j == 1 || pal[j + 1][i - 1]) {
                    pal[j][i] = true;
                }
            }
        }
        for i in 1..s.len() - 1 {
            for j in i + 1..s.len() {
                if pal[0][i - 1] && pal[i][j - 1] && pal[j][s.len() - 1] {
                    return true;
                }
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check_partitioning() {
        assert_eq!(Solution::check_partitioning("abcbdd".to_string()), true);
        assert_eq!(Solution::check_partitioning("bcbddxy".to_string()), false);
    }
}
