// https://leetcode.com/problems/find-maximum-removals-from-source-string/
// 3316. Find Maximum Number of Removals From Source String
pub struct Solution;
impl Solution {
    pub fn max_removals(source: String, pattern: String, target_indices: Vec<i32>) -> i32 {
        let source = source.chars().collect::<Vec<char>>();
        let pattern = pattern.chars().collect::<Vec<char>>();
        let mut timap = vec![0; source.len()];
        for i in target_indices {
            timap[i as usize] += 1;
        }
        let pl = pattern.len();
        let mut dp = vec![i32::MIN; pl + 1];
        dp[pl] = 0;
        for i in (0..source.len()).rev() {
            for j in 0..=pl {
                dp[j] += timap[i];
                if j < pl && source[i] == pattern[j] {
                    dp[j] = dp[j].max(dp[j + 1]);
                }
            }
        }
        dp[0]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_removals() {
        assert_eq!(
            Solution::max_removals("abbaa".to_string(), "aba".to_string(), vec![0, 1, 2]),
            1
        );
        assert_eq!(
            Solution::max_removals("bcda".to_string(), "d".to_string(), vec![0, 3]),
            2
        );
        assert_eq!(
            Solution::max_removals("dda".to_string(), "dda".to_string(), vec![0, 1, 2]),
            0
        );
        assert_eq!(
            Solution::max_removals("yeyeykyded".to_string(), "yeyyd".to_string(), vec![0, 2, 3, 4]),
            2
        );
    }
}
