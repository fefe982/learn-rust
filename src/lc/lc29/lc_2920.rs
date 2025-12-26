// https://leetcode.com/problems/maximum-points-after-collecting-coins-from-all-nodes/
// 2920. Maximum Points After Collecting Coins From All Nodes
pub struct Solution;
impl Solution {
    fn collect(
        g: &Vec<Vec<usize>>,
        coins: &Vec<i32>,
        cache: &mut Vec<Vec<i32>>,
        p: usize,
        n: usize,
        k: i32,
        halves: usize,
    ) -> i32 {
        if halves >= 14 {
            return 0;
        }
        if cache[n][halves] >= 0 {
            return cache[n][halves];
        }
        let val = coins[n] / (1 << halves);
        let mut ret1 = val - k;
        for &next in &g[n] {
            if next == p {
                continue;
            }
            ret1 += Solution::collect(g, coins, cache, n, next, k, halves);
        }
        if val - k < val / 2 {
            let mut ret2 = val / 2;
            for &next in &g[n] {
                if next == p {
                    continue;
                }
                ret2 += Solution::collect(g, coins, cache, n, next, k, halves + 1);
            }
            ret1 = ret1.max(ret2);
        }
        cache[n][halves] = ret1;
        ret1
    }
    pub fn maximum_points(edges: Vec<Vec<i32>>, coins: Vec<i32>, k: i32) -> i32 {
        let mut g = vec![vec![]; coins.len()];
        for e in edges {
            g[e[0] as usize].push(e[1] as usize);
            g[e[1] as usize].push(e[0] as usize);
        }
        Solution::collect(&g, &coins, &mut vec![vec![-1; 14]; coins.len()], usize::MAX, 0, k, 0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn maximum_points() {
        assert_eq!(
            Solution::maximum_points(vec_vec![[0, 1], [1, 2], [2, 3]], vec![10, 10, 3, 3], 5),
            11
        );
        assert_eq!(Solution::maximum_points(vec_vec![[0, 1], [0, 2]], vec![8, 4, 4], 0), 16);
    }
}
