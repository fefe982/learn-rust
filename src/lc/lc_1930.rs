// https://leetcode.com/problems/unique-length-3-palindromic-subsequences/
// 1930. Unique Length-3 Palindromic Subsequences
pub struct Solution;
impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let mut pos = vec![vec![]; 26];
        for (i, c) in s.chars().enumerate() {
            let idx = c as usize - 'a' as usize;
            pos[idx].push(i);
        }
        let mut ans = 0;
        for i in 0..26 {
            if pos[i].len() < 2 {
                continue;
            }
            if pos[i].len() >= 3 {
                ans += 1;
            }
            for j in 0..26 {
                if pos[j].len() == 0 || j == i {
                    continue;
                }
                if pos[j].partition_point(|&x| x < pos[i][0])
                    != pos[j].partition_point(|&x| x < *pos[i].last().unwrap())
                {
                    ans += 1;
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
    fn test_count_palindromic_subsequence() {
        assert_eq!(
            Solution::count_palindromic_subsequence("tlpjzdmtwderpkpmgoyrcxttiheassztncqvnfjeyxxp".to_owned()),
            161
        );
        assert_eq!(Solution::count_palindromic_subsequence("aabca".to_owned()), 3);
        assert_eq!(Solution::count_palindromic_subsequence("adc".to_owned()), 0);
        assert_eq!(Solution::count_palindromic_subsequence("bbcbaba".to_owned()), 4);
    }
}
