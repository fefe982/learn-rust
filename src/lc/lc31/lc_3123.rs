// https://leetcode.com/problems/find-edges-in-shortest-paths/
// 3123. Find Edges in Shortest Paths
pub struct Solution;
impl Solution {
    pub fn find_answer(n: i32, edges: Vec<Vec<i32>>) -> Vec<bool> {
        let mut ans = vec![false; edges.len()];
        let mut g = vec![vec![]; n as usize];
        for (i, e) in edges.into_iter().enumerate() {
            g[e[0] as usize].push((e[1] as usize, e[2], i));
            g[e[1] as usize].push((e[0] as usize, e[2], i));
        }
        let n = n as usize;
        let mut back = vec![(i64::MAX, vec![]); n];
        let mut q = std::collections::BinaryHeap::new();
        q.push((std::cmp::Reverse(0), 0, usize::MAX, usize::MAX));
        while let Some((std::cmp::Reverse(len), node, from, edge)) = q.pop() {
            match len.cmp(&back[node].0) {
                std::cmp::Ordering::Less => {
                    back[node] = (len, vec![(from, edge)]);
                    for &(next, d, e) in &g[node] {
                        if len + d as i64 <= back[next].0 {
                            q.push((std::cmp::Reverse(len + d as i64), next, node, e));
                        }
                    }
                }
                std::cmp::Ordering::Equal => {
                    back[node].1.push((from, edge));
                }
                std::cmp::Ordering::Greater => {}
            }
            if len > back[n - 1].0 {
                break;
            }
        }
        if back[n - 1].1.len() > 0 {
            let mut q = vec![n - 1];
            while let Some(node) = q.pop() {
                if node == 0 || back[node].1.len() == 0 {
                    continue;
                }
                if ans[back[node].1[0].1] {
                    continue;
                }
                for &(from, edge) in &back[node].1 {
                    ans[edge] = true;
                    (q).push(from);
                }
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn find_answer() {
        assert_eq!(
            Solution::find_answer(
                6,
                vec_vec![
                    [0, 1, 4],
                    [0, 2, 1],
                    [1, 3, 2],
                    [1, 4, 3],
                    [1, 5, 1],
                    [2, 3, 1],
                    [3, 5, 3],
                    [4, 5, 2]
                ]
            ),
            [true, true, true, false, true, true, true, false]
        );
        assert_eq!(
            Solution::find_answer(4, vec_vec![[2, 0, 1], [0, 1, 1], [0, 3, 4], [3, 2, 2]]),
            [true, false, false, true]
        );
    }
}
