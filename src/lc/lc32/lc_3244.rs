// https://leetcode.com/problems/shortest-distance-after-road-addition-queries-ii/
// 3244. Shortest Distance After Road Addition Queries II
pub struct Solution;
impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = Vec::with_capacity(queries.len());
        let mut edge = std::collections::BTreeMap::<i32, i32>::new();
        let mut len = n - 1;
        for q in queries {
            if let Some(&e) = edge.get(&q[0]) {
                if e >= q[1] {
                    ans.push(len);
                    continue;
                }
            }
            if let Some((_, &v)) = edge.range(..q[0]).rev().next() {
                if v >= q[1] {
                    ans.push(len);
                    continue;
                }
            }
            let remove = edge
                .range(q[0]..)
                .take_while(|&(_, &v)| v <= q[1])
                .map(|(&k, _)| k)
                .collect::<Vec<_>>();
            for r in remove {
                len += edge.remove(&r).unwrap() - r - 1;
            }
            len -= q[1] - q[0] - 1;
            edge.insert(q[0], q[1]);
            ans.push(len);
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
            Solution::shortest_distance_after_queries(5, vec_vec![[1, 4], [2, 4]]),
            [2, 2]
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
