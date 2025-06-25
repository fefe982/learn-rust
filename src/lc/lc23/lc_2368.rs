// https://leetcode.com/problems/reachable-nodes-with-restrictions/
// 2368. Reachable Nodes With Restrictions
pub struct Solution;
impl Solution {
    pub fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
        let mut g = vec![vec![]; n as usize];
        let mut r = vec![false; n as usize];
        for e in edges {
            g[e[0] as usize].push(e[1] as usize);
            g[e[1] as usize].push(e[0] as usize);
        }
        for ir in restricted {
            r[ir as usize] = true;
        }
        let mut cnt = 1;
        let mut q = vec![(n as usize, 0)];
        while let Some((p, c)) = q.pop() {
            for &n in &g[c] {
                if n == p || r[n] {
                    continue;
                }
                cnt += 1;
                q.push((c, n));
            }
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_reachable_nodes() {
        assert_eq!(
            Solution::reachable_nodes(7, vec_vec![[0, 1], [1, 2], [3, 1], [4, 0], [0, 5], [5, 6]], vec![4, 5]),
            4
        );
        assert_eq!(
            Solution::reachable_nodes(
                7,
                vec_vec![[0, 1], [0, 2], [0, 5], [0, 4], [3, 2], [6, 5]],
                vec![4, 2, 1]
            ),
            3
        );
    }
}
