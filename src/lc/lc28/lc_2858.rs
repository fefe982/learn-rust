// https://leetcode.com/problems/minimum-edge-reversals-so-every-node-is-reachable/
// 2858. Minimum Edge Reversals to Make Every Node Reachable
pub struct Solution;
impl Solution {
    fn swap(g: &Vec<Vec<(usize, bool)>>, p: usize, n: usize, pn_forward: bool, nswap: i32, vec: &mut Vec<i32>) {
        let thiswap = if pn_forward { nswap + 1 } else { nswap - 1 };
        vec[n] = thiswap;
        for &(next, forward) in &g[n] {
            if next != p {
                Self::swap(g, n, next, forward, thiswap, vec);
            }
        }
    }
    fn traverse(g: &Vec<Vec<(usize, bool)>>, p: usize, n: usize) -> i32 {
        let mut res = 0;
        for &(next, forward) in &g[n] {
            if next != p {
                if !forward {
                    res += 1;
                }
                res += Self::traverse(g, n, next);
            }
        }
        res
    }
    pub fn min_edge_reversals(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut g = vec![vec![]; n as usize];
        for edge in edges {
            g[edge[0] as usize].push((edge[1] as usize, true));
            g[edge[1] as usize].push((edge[0] as usize, false));
        }
        let nswap = Self::traverse(&g, usize::MAX, 0);
        let mut res = vec![0; n as usize];
        res[0] = nswap;
        Self::swap(&g, usize::MAX, 0, false, nswap + 1, &mut res);
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_min_edge_reversals() {
        assert_eq!(
            Solution::min_edge_reversals(4, vec_vec![[2, 0], [2, 1], [1, 3]]),
            [1, 1, 0, 2]
        );
        assert_eq!(Solution::min_edge_reversals(3, vec_vec![[1, 2], [2, 0]]), [2, 0, 1]);
    }
}
