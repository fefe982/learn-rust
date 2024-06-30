// https://leetcode.com/problems/minimum-weighted-subgraph-with-the-required-paths/
// 2203. Minimum Weighted Subgraph With the Required Paths
pub struct Solution;
impl Solution {
    fn spath(g: &Vec<Vec<(usize, i32)>>, src: usize) -> Vec<i64> {
        let mut dist = vec![i64::MAX / 3; g.len()];
        let mut h = std::collections::BinaryHeap::new();
        h.push((std::cmp::Reverse(0), src));
        while let Some((std::cmp::Reverse(d), n)) = h.pop() {
            if dist[n] <= d {
                continue;
            }
            dist[n] = d;
            for (m, w) in g[n].iter() {
                h.push((std::cmp::Reverse(d + *w as i64), *m));
            }
        }
        dist
    }
    pub fn minimum_weight(n: i32, edges: Vec<Vec<i32>>, src1: i32, src2: i32, dest: i32) -> i64 {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        for e in edges.iter() {
            g[e[0] as usize].push((e[1] as usize, e[2]));
        }
        let src1 = src1 as usize;
        let src2 = src2 as usize;
        let src1dist = Solution::spath(&g, src1);
        let src2dist = Solution::spath(&g, src2);
        g.iter_mut().for_each(|e| e.clear());
        for e in edges {
            g[e[1] as usize].push((e[0] as usize, e[2]));
        }
        let dest = dest as usize;
        let destdist = Solution::spath(&g, dest);
        let mut ans = i64::MAX;
        for i in 0..n {
            ans = ans.min(src1dist[i] + src2dist[i] + destdist[i]);
        }
        if ans >= i64::MAX / 3 {
            -1
        } else {
            ans
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_minimum_weight() {
        assert_eq!(
            Solution::minimum_weight(3, vec_vec![[0, 2, 10], [1, 2, 10], [1, 0, 1]], 0, 1, 2),
            11
        );
        assert_eq!(
            Solution::minimum_weight(
                5,
                vec_vec![[4, 2, 20], [4, 3, 46], [0, 1, 15], [0, 1, 43], [0, 1, 32], [3, 1, 13]],
                0,
                4,
                1
            ),
            74
        );
        assert_eq!(
            Solution::minimum_weight(
                6,
                vec_vec![
                    [0, 2, 2],
                    [0, 5, 6],
                    [1, 0, 3],
                    [1, 4, 5],
                    [2, 1, 1],
                    [2, 3, 3],
                    [2, 3, 4],
                    [3, 4, 2],
                    [4, 5, 1]
                ],
                0,
                1,
                5
            ),
            9
        );
        assert_eq!(Solution::minimum_weight(3, vec_vec![[0, 1, 1], [2, 1, 1]], 0, 1, 2), -1);
    }
}
