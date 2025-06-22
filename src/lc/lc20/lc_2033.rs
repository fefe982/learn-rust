// https://leetcode.com/problems/minimum-operations-to-make-a-uni-value-grid/
// 2033. Minimum Operations to Make a Uni-Value Grid
pub struct Solution;
impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let mut v = grid.into_iter().map(|v| v.into_iter()).flatten().collect::<Vec<_>>();
        v.sort_unstable();
        let n = v.len();
        let mut ans = 0;
        let mid = v[n / 2];
        let r = mid % x;
        for i in 0..n {
            if v[i] % x != r {
                return -1;
            }
            ans += (v[i] - mid).abs() / x;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_min_operations() {
        assert_eq!(Solution::min_operations(vec_vec![[2, 4], [6, 8]], 2), 4);
        assert_eq!(Solution::min_operations(vec_vec![[1, 5], [2, 3]], 1), 5);
        assert_eq!(Solution::min_operations(vec_vec![[1, 2], [3, 4]], 2), -1);
    }
}
