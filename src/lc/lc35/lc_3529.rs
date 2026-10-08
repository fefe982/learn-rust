// https://leetcode.com/problems/count-cells-in-overlapping-horizontal-and-vertical-substrings/
// 3529. Count Cells in Overlapping Horizontal and Vertical Substrings
pub struct Solution;
impl Solution {
    fn get_lps(s: &[char]) -> Vec<usize> {
        let n = s.len();
        let mut lps = vec![0; n];
        for i in 1..n {
            let mut j = lps[i - 1];
            while j > 0 && s[i] != s[j] {
                j = lps[j - 1];
            }
            if s[i] == s[j] {
                j += 1;
            }
            lps[i] = j;
        }
        lps
    }
    fn scan<F>(haystack: &F, len: usize, pattern: &[char], lps: &[usize]) -> Vec<i32>
    where
        F: Fn(usize) -> char,
    {
        let mut res = vec![0; len + 1];
        let mut j = 0;
        for i in 0..len {
            while j > 0 && haystack(i) != pattern[j] {
                j = lps[j - 1];
            }
            if haystack(i) == pattern[j] {
                j += 1;
            }
            if j == pattern.len() {
                res[i + 1 - pattern.len()] += 1;
                res[i + 1] -= 1;
                j = lps[j - 1];
            }
        }
        res
    }
    pub fn count_cells(grid: Vec<Vec<char>>, pattern: String) -> i32 {
        let pattern = pattern.chars().collect::<Vec<char>>();
        let lps = Self::get_lps(&pattern);
        let g1 = grid.len();
        let g2 = grid[0].len();
        let scan1 = Self::scan(&|i| grid[i / g2][i % g2], g1 * g2, &pattern, &lps);
        let scan2 = Self::scan(&|i| grid[i % g1][i / g1], g1 * g2, &pattern, &lps);
        let mut mark = vec![vec![0; g2]; g1];
        let mut c1 = 0;
        let mut c2 = 0;
        for i in 0..g1 * g2 {
            c1 += scan1[i];
            c2 += scan2[i];
            if c1 > 0 {
                mark[i / g2][i % g2] += 1;
            }
            if c2 > 0 {
                mark[i % g1][i / g1] += 1;
            }
        }
        let mut count = 0;
        for i in 0..g1 {
            for j in 0..g2 {
                if mark[i][j] > 1 {
                    count += 1;
                }
            }
        }
        count
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn count_cells() {
        assert_eq!(
            Solution::count_cells(
                vec_vec_chr![
                    ["a", "a", "c", "c"],
                    ["b", "b", "b", "c"],
                    ["a", "a", "b", "a"],
                    ["c", "a", "a", "c"],
                    ["a", "a", "b", "a"]
                ],
                "abaca".to_string()
            ),
            1
        );
        assert_eq!(
            Solution::count_cells(
                vec_vec_chr![
                    ["c", "a", "a", "a"],
                    ["a", "a", "b", "a"],
                    ["b", "b", "a", "a"],
                    ["a", "a", "b", "a"]
                ],
                "aba".to_string()
            ),
            4
        );
        assert_eq!(Solution::count_cells(vec_vec_chr![["a"]], "a".to_string()), 1);
    }
}
