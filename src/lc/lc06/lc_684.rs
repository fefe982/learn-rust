// https://leetcode.com/problems/redundant-connection/
// 684. Redundant Connection
pub struct Solution;
impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut g = vec![vec![]; edges.len() + 1];
        let mut degree = vec![0; edges.len() + 1];
        for (i, edge) in edges.iter().enumerate() {
            g[edge[0] as usize].push((edge[1] as usize, i));
            g[edge[1] as usize].push((edge[0] as usize, i));
            degree[edge[0] as usize] += 1;
            degree[edge[1] as usize] += 1;
        }
        let mut q = vec![];
        for (i, d) in degree.iter().enumerate() {
            if *d == 1 {
                q.push(i);
            }
        }
        while let Some(n) = q.pop() {
            if degree[n] == 0 {
                continue;
            }
            degree[n] = 0;
            for &(m, _) in g[n].iter() {
                degree[m] -= 1;
                if degree[m] == 1 {
                    q.push(m);
                }
            }
        }
        let mut node = 0;
        for (i, d) in degree.iter().enumerate() {
            if *d == 2 {
                node = i;
                break;
            }
        }
        let mut cnode = node;
        let mut pnode = 0;
        let mut res = 0;
        loop {
            for &(n, i) in g[cnode].iter() {
                if n != pnode && degree[n] == 2 {
                    pnode = cnode;
                    cnode = n;
                    res = res.max(i);
                    break;
                }
            }
            if cnode == node {
                return edges[res].clone();
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn find_redundant_connection() {
        assert_eq!(
            Solution::find_redundant_connection(vec_vec![[1, 2], [1, 3], [2, 3]]),
            vec![2, 3]
        );
        assert_eq!(
            Solution::find_redundant_connection(vec_vec![[1, 2], [2, 3], [3, 4], [1, 4], [1, 5]]),
            vec![1, 4]
        );
    }
}
