// https://leetcode.com/problems/number-of-ways-to-arrive-at-destination/
// 1976. Number of Ways to Arrive at Destination
pub struct Solution;
impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let mut g = vec![vec![]; n as usize];
        for r in roads {
            g[r[0] as usize].push((r[1] as usize, r[2] as i64));
            g[r[1] as usize].push((r[0] as usize, r[2] as i64));
        }
        let n = n as usize;
        let mut dist = vec![i64::MAX; n];
        dist[0] = 0;
        let mut count = vec![0; n];
        count[0] = 1;
        let mut visit = vec![false; n];
        let mut q = std::collections::BinaryHeap::new();
        q.push(std::cmp::Reverse((0, 0)));
        while let Some(std::cmp::Reverse((d, node))) = q.pop() {
            if visit[node] {
                continue;
            }
            if node == n - 1 {
                break;
            }
            visit[node] = true;
            let c = count[node];
            for &(n, nd) in &g[node] {
                let d = d + nd;
                if d > dist[n] {
                    continue;
                }
                if d < dist[n] {
                    dist[n] = d;
                    count[n] = c;
                    q.push(std::cmp::Reverse((d, n)));
                } else {
                    count[n] = (count[n] + c) % 1_0000_0000_7;
                }
            }
        }
        count[n - 1] as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_count_paths() {
        assert_eq!(
            Solution::count_paths(
                7,
                vec_vec![
                    [0, 6, 7],
                    [0, 1, 2],
                    [1, 2, 3],
                    [1, 3, 3],
                    [6, 3, 3],
                    [3, 5, 1],
                    [6, 5, 1],
                    [2, 5, 1],
                    [0, 4, 5],
                    [4, 6, 2]
                ]
            ),
            4
        );
        assert_eq!(Solution::count_paths(2, vec_vec![[1, 0, 10]]), 1);
    }
}
