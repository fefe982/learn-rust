// https://leetcode.com/problems/maximize-spanning-tree-stability-with-upgrades/
// 3600. Maximize Spanning Tree Stability With Upgrades
pub struct Solution;
impl Solution {
    pub fn max_stability(n: i32, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = n as usize;
        let mut v = (0..n).collect::<Vec<usize>>();
        fn find(v: &mut Vec<usize>, x: usize) -> usize {
            if v[x] != x {
                v[x] = find(v, v[x]);
            }
            v[x]
        }
        fn union(v: &mut Vec<usize>, x: usize, y: usize) -> bool {
            let x = find(v, x);
            let y = find(v, y);
            v[x] = y;
            x != y
        }
        let mut min = i32::MAX;
        let mut nc = n;
        for edge in &edges {
            if edge[3] == 1 {
                if !union(&mut v, edge[0] as usize, edge[1] as usize) {
                    return -1;
                }
                nc -= 1;
                min = min.min(edge[2]);
            }
        }
        if nc == 1 {
            return min;
        }
        let mut edges = edges.into_iter().filter(|x| x[3] == 0).collect::<Vec<_>>();
        edges.sort_unstable_by_key(|x| x[2]);
        let k = k as usize;
        for edge in edges.into_iter().rev() {
            if union(&mut v, edge[0] as usize, edge[1] as usize) {
                nc -= 1;
                let mut w = edge[2];
                if nc <= k {
                    w *= 2;
                }
                min = min.min(w);
                if nc == 1 {
                    return min;
                }
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn max_stability() {
        assert_eq!(Solution::max_stability(2, vec_vec![[0, 1, 87487, 0]], 0), 87487);
        assert_eq!(Solution::max_stability(3, vec_vec![[0, 1, 2, 1], [1, 2, 3, 0]], 1), 2);
        assert_eq!(
            Solution::max_stability(3, vec_vec![[0, 1, 4, 0], [1, 2, 3, 0], [0, 2, 1, 0]], 2),
            6
        );
        assert_eq!(
            Solution::max_stability(3, vec_vec![[0, 1, 1, 1], [1, 2, 1, 1], [2, 0, 1, 1]], 0),
            -1
        );
    }
}
