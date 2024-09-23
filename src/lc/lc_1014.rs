// https://leetcode.com/problems/best-sightseeing-pair/
// 1014. Best Sightseeing Pair
pub struct Solution;
impl Solution {
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        let mut suffix = values.clone();
        let n = values.len();
        for i in (1..n).rev() {
            suffix[i] -= i as i32;
            if i + 1 < n {
                suffix[i] = suffix[i].max(suffix[i + 1]);
            }
        }
        let mut m = i32::MIN;
        let mut prefix = i32::MIN;
        for i in 0..n - 1 {
            prefix = prefix.max(values[i] + i as i32);
            m = m.max(prefix + suffix[i + 1]);
        }
        m
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_score_sightseeing_pair() {
        assert_eq!(Solution::max_score_sightseeing_pair(vec![8, 1, 5, 2, 6]), 11);
        assert_eq!(Solution::max_score_sightseeing_pair(vec![1, 2]), 2);
    }
}
