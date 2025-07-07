// https://leetcode.com/problems/minimum-degree-of-a-connected-trio-in-a-graph/
// 1761. Minimum Degree of a Connected Trio in a Graph
pub struct Solution;
impl Solution {
    pub fn min_trio_degree(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut min = i32::MAX;
        let n = n as usize;
        let mut d = vec![0; n];
        let mut g = vec![std::collections::HashSet::<usize>::new(); n];
        for e in &edges {
            let x = e[0] as usize - 1;
            let y = e[1] as usize - 1;
            d[x] += 1;
            d[y] += 1;
            g[x].insert(y);
            g[y].insert(x);
        }
        let mut h = vec![vec![]; n];
        for e in edges {
            let x = e[0] as usize - 1;
            let y = e[1] as usize - 1;
            if d[x] < d[y] || (d[x] == d[y] && x < y) {
                h[x].push(y);
            } else {
                h[y].push(x);
            }
        }
        for (i, v) in h.into_iter().enumerate() {
            if v.len() < 2 {
                continue;
            }
            for j in 0..v.len() - 1 {
                let x = v[j];
                let sx = &g[x];
                for k in j + 1..v.len() {
                    let y = v[k];
                    if sx.contains(&y) {
                        min = min.min(d[i] + d[x] + d[y] - 6);
                    }
                }
            }
        }
        if min == i32::MAX {
            -1
        } else {
            min
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn min_trio_degree() {
        assert_eq!(
            Solution::min_trio_degree(6, vec_vec![[1, 2], [1, 3], [3, 2], [4, 1], [5, 2], [3, 6]]),
            3
        );
        assert_eq!(
            Solution::min_trio_degree(
                7,
                vec_vec![
                    [1, 3],
                    [4, 1],
                    [4, 3],
                    [2, 5],
                    [5, 6],
                    [6, 7],
                    [7, 5],
                    [2, 6]
                ]
            ),
            0
        );
    }
}
