// https://leetcode.com/problems/maximum-total-importance-of-roads/
// 2285. Maximum Total Importance of Roads
pub struct Solution;
impl Solution {
    pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
        let n = n as usize;
        let mut deg = vec![0; n];
        for r in roads {
            deg[r[0] as usize] += 1;
            deg[r[1] as usize] += 1;
        }
        deg.sort_unstable();
        deg.into_iter()
            .enumerate()
            .map(|(i, d)| (i + 1) as i64 * d as i64)
            .sum()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_maximum_importance() {
        assert_eq!(
            Solution::maximum_importance(5, vec_vec![[0, 1], [1, 2], [2, 3], [0, 2], [1, 3], [2, 4]]),
            43
        );
        assert_eq!(Solution::maximum_importance(5, vec_vec![[0, 3], [2, 4], [1, 3]]), 20);
    }
}
