// https://leetcode.com/problems/shortest-cycle-in-a-graph/
// 2608. Shortest Cycle in a Graph
pub struct Solution;
impl Solution {
    pub fn find_shortest_cycle(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        for e in edges {
            g[e[0] as usize].push(e[1] as usize);
            g[e[1] as usize].push(e[0] as usize);
        }
        let mut ans = i32::MAX;
        let mut dist = vec![-1; n];
        'node: for i in 0..n {
            let mut q = std::collections::VecDeque::new();
            dist[i] = (i * n) as i32;
            q.push_back((i, dist[i]));
            while let Some((u, d)) = q.pop_front() {
                if d * 2 - dist[i] * 2 + 1 >= ans {
                    break;
                }
                for &v in &g[u] {
                    if v > i && dist[v] < dist[i] {
                        dist[v] = d + 1;
                        q.push_back((v, d + 1));
                    } else if dist[u] == dist[v] {
                        ans = ans.min(dist[u] + dist[v] - dist[i] * 2 + 1);
                        continue 'node;
                    } else if dist[v] > dist[u] {
                        ans = ans.min(dist[u] + dist[v] - dist[i] * 2 + 1);
                        break;
                    }
                }
            }
        }
        if ans == i32::MAX {
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
    fn test_find_shortest_cycle() {
        assert_eq!(Solution::find_shortest_cycle(6, vec_vec![[1, 3], [3, 5], [5, 1]]), 3);
        assert_eq!(
            Solution::find_shortest_cycle(6, vec_vec![[4, 1], [5, 1], [3, 2], [5, 0], [4, 0], [3, 0], [2, 1]]),
            4
        );
        assert_eq!(
            Solution::find_shortest_cycle(7, vec_vec![[0, 1], [1, 2], [2, 0], [3, 4], [4, 5], [5, 6], [6, 3]]),
            3
        );
        assert_eq!(Solution::find_shortest_cycle(4, vec_vec![[0, 1], [0, 2]]), -1);
    }
}
