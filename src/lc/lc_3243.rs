// https://leetcode.com/problems/shortest-distance-after-road-addition-queries-i/
// 3243. Shortest Distance After Road Addition Queries I
pub struct Solution;
impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut edges = (1..n as usize).map(|i| vec![i]).collect::<Vec<_>>();
        let mut dist = (0..n).collect::<Vec<_>>();
        let mut ans = Vec::with_capacity(queries.len());
        for q in queries {
            edges[q[0] as usize].push(q[1] as usize);
            let mut h = std::collections::BinaryHeap::new();
            if dist[q[1] as usize] > dist[q[0] as usize] + 1 {
                dist[q[1] as usize] = dist[q[0] as usize] + 1;
                h.push((std::cmp::Reverse(dist[q[1] as usize]), q[1] as usize));
            }
            while let Some((std::cmp::Reverse(d), i)) = h.pop() {
                if i == n as usize - 1 {
                    break;
                }
                for &j in &edges[i] {
                    if dist[j] > d + 1 {
                        dist[j] = d + 1;
                        h.push((std::cmp::Reverse(d + 1), j));
                    }
                }
            }
            ans.push(dist[n as usize - 1]);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_shortest_distance_after_queries() {
        assert_eq!(
            Solution::shortest_distance_after_queries(5, vec_vec![[1, 3], [2, 4]]),
            [3, 3]
        );
        assert_eq!(
            Solution::shortest_distance_after_queries(5, vec_vec![[2, 4], [0, 2], [0, 4]]),
            [3, 2, 1]
        );
        assert_eq!(
            Solution::shortest_distance_after_queries(4, vec_vec![[0, 3], [0, 2]]),
            [1, 1]
        );
    }
}
