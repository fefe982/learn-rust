// https://leetcode.com/problems/find-critical-and-pseudo-critical-edges-in-minimum-spanning-tree/description/
// 1489. Find Critical and Pseudo-Critical Edges in Minimum Spanning Tree
pub struct Solution;
impl Solution {
    fn get_root(f: &Vec<usize>, mut i: usize) -> usize {
        while f[i] != i {
            i = f[i];
        }
        i
    }
    fn find_path(
        mut f: Vec<usize>,
        edges: &Vec<(i32, usize, usize, usize)>,
        include: usize,
        exclude: usize,
        n: usize,
    ) -> i32 {
        let mut c = 0;
        let mut s = 0;
        if include < edges.len() {
            f[edges[include].1] = f[edges[include].2];
            c = 1;
            s += edges[include].0;
        }
        for (i, &(len, from, to, _)) in edges.iter().enumerate() {
            if i == exclude {
                continue;
            }
            let rto = Self::get_root(&f, to);
            let rfrom = Self::get_root(&f, from);
            if rfrom != rto {
                c += 1;
                s += len;
                f[rfrom] = rto;
            }
        }
        if c + 1 < n {
            i32::MAX
        } else {
            s
        }
    }
    pub fn find_critical_and_pseudo_critical_edges(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut f = Vec::with_capacity(n);
        for i in 0..n {
            f.push(i);
        }
        let mut edges = edges
            .into_iter()
            .enumerate()
            .map(|(i, v)| (v[2], v[0] as usize, v[1] as usize, i))
            .collect::<Vec<(i32, usize, usize, usize)>>();
        edges.sort_unstable();
        let min = Self::find_path(f.clone(), &edges, usize::MAX, usize::MAX, n);
        let mut res = vec![vec![]; 2];
        let ne = edges.len();
        for i in 0..ne {
            if Self::find_path(f.clone(), &edges, usize::MAX, i, n) > min {
                res[0].push(edges[i].3 as i32);
            } else if Self::find_path(f.clone(), &edges, i, usize::MAX, n) == min {
                res[1].push(edges[i].3 as i32);
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn find_critical_and_pseudo_critical_edges() {
        assert_eq!(
            Solution::find_critical_and_pseudo_critical_edges(
                5,
                vec_vec![
                    [0, 1, 1],
                    [1, 2, 1],
                    [2, 3, 2],
                    [0, 3, 2],
                    [0, 4, 3],
                    [3, 4, 3],
                    [1, 4, 6]
                ]
            ),
            vec_vec![[0, 1], [3, 2, 4, 5]]
        );
        assert_eq!(
            Solution::find_critical_and_pseudo_critical_edges(
                4,
                vec_vec![[0, 1, 1], [1, 2, 1], [2, 3, 1], [0, 3, 1]]
            ),
            vec_vec![[], [0, 3, 1, 2]]
        );
    }
}
