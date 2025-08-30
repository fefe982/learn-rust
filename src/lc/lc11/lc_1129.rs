// https://leetcode.com/problems/shortest-path-with-alternating-colors/
// 1129. Shortest Path with Alternating Colors
pub struct Solution;
impl Solution {
    pub fn shortest_alternating_paths(n: i32, red_edges: Vec<Vec<i32>>, blue_edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut res = vec![[i32::MAX; 2]; n];
        let mut g = vec![vec![vec![]; n]; 2];
        for e in red_edges {
            g[0][e[0] as usize].push(e[1] as usize);
        }
        for e in blue_edges {
            g[1][e[0] as usize].push(e[1] as usize);
        }
        res[0] = [0, 0];
        let mut q = std::collections::VecDeque::new();
        q.push_back((0, 0));
        q.push_back((0, 1));
        while let Some((u, c)) = q.pop_front() {
            let nc = 1 - c;
            for &v in &g[nc][u] {
                if res[v][nc] == i32::MAX {
                    res[v][nc] = res[u][c] + 1;
                    q.push_back((v, nc));
                }
            }
        }
        return res
            .into_iter()
            .map(|x| {
                let v = x[0].min(x[1]);
                if v == i32::MAX {
                    -1
                } else {
                    v
                }
            })
            .collect();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn shortest_alternating_paths() {
        assert_eq!(
            Solution::shortest_alternating_paths(3, vec_vec![[0, 1], [1, 2]], vec_vec![]),
            vec![0, 1, -1]
        );
        assert_eq!(
            Solution::shortest_alternating_paths(3, vec_vec![[0, 1]], vec_vec![[2, 1]]),
            vec![0, 1, -1]
        );
    }
}
