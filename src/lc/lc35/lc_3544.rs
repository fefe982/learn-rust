// https://leetcode.com/problems/subtree-inversion-sum/
// 3544. Subtree Inversion Sum
pub struct Solution;
impl Solution {
    fn dfs(
        g: &Vec<Vec<usize>>,
        nums: &Vec<i32>,
        k: usize,
        cache: &mut Vec<Vec<Vec<i64>>>,
        u: usize,
        p: usize,
        parity: usize,
        d: usize,
    ) -> i64 {
        if cache[parity][d][u] != i64::MIN {
            return cache[parity][d][u];
        }
        let mul = if parity == 1 { -1 } else { 1 };
        let mut inv = (-nums[u] * mul) as i64;
        let mut org = (nums[u] * mul) as i64;
        for &v in &g[u] {
            if v == p {
                continue;
            }
            if d == 0 {
                inv += Self::dfs(g, nums, k, cache, v, u, 1 - parity, k - 1);
            }
            org += Self::dfs(g, nums, k, cache, v, u, parity, d.saturating_sub(1));
        }
        if d == 0 {
            org = org.max(inv);
        }
        cache[parity][d][u] = org;
        org
    }
    pub fn subtree_inversion_sum(edges: Vec<Vec<i32>>, nums: Vec<i32>, k: i32) -> i64 {
        let n = edges.len() + 1;
        let mut g = vec![vec![]; n];
        for e in edges {
            g[e[0] as usize].push(e[1] as usize);
            g[e[1] as usize].push(e[0] as usize);
        }
        let k = k as usize;
        let mut cache = vec![vec![vec![i64::MIN; n]; k]; 2];
        Self::dfs(&g, &nums, k, &mut cache, 0, n, 0, 0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn subtree_inversion_sum() {
        assert_eq!(
            Solution::subtree_inversion_sum(
                vec_vec![[0, 1], [0, 2], [1, 3], [1, 4], [2, 5], [2, 6]],
                vec![4, -8, -6, 3, 7, -2, 5],
                2
            ),
            27
        );
        assert_eq!(
            Solution::subtree_inversion_sum(vec_vec![[0, 1], [1, 2], [2, 3], [3, 4]], vec![-1, 3, -2, 4, -5], 2),
            9
        );
        assert_eq!(
            Solution::subtree_inversion_sum(vec_vec![[0, 1], [0, 2]], vec![0, -1, -2], 3),
            3
        );
    }
}
