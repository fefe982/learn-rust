// https://leetcode.com/problems/total-sum-of-interaction-cost-in-tree-groups/
// 3786. Total Sum of Interaction Costs in Tree Groups
pub struct Solution;
impl Solution {
    fn dfs(
        g: &Vec<Vec<usize>>,
        sg: &Vec<(usize, i64)>,
        group: &Vec<i32>,
        u: usize,
        p: usize,
        sum: &mut i64,
    ) -> [i64; 21] {
        let mut cnt = [0; 21];
        for &v in &g[u] {
            if v != p {
                let cc = Self::dfs(g, sg, group, v, u, sum);
                for &(i, _) in sg {
                    cnt[i] += cc[i];
                }
            }
        }
        cnt[group[u] as usize] += 1;
        if p != usize::MAX {
            for &(i, c) in sg {
                *sum += (c - cnt[i]) * cnt[i];
            }
        }
        cnt
    }
    pub fn interaction_costs(n: i32, edges: Vec<Vec<i32>>, group: Vec<i32>) -> i64 {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        for e in edges {
            g[e[0] as usize].push(e[1] as usize);
            g[e[1] as usize].push(e[0] as usize);
        }
        let mut sg = std::collections::HashMap::new();
        for &g in &group {
            *sg.entry(g).or_insert(0) += 1;
        }
        let cg = sg.into_iter().map(|(k, v)| (k as usize, v as i64)).collect::<Vec<_>>();
        let mut sum = 0;
        Self::dfs(&g, &cg, &group, 0, usize::MAX, &mut sum);
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn interaction_costs() {
        assert_eq!(
            Solution::interaction_costs(4, vec_vec![[0, 1], [0, 2], [0, 3]], vec![1, 1, 4, 4]),
            3
        );
        assert_eq!(Solution::interaction_costs(2, vec_vec![[0, 1]], vec![9, 8]), 0);
    }
}
