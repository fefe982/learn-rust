// https://leetcode.com/problems/minimum-threshold-path-with-limited-heavy-edges/
// 3924. Minimum Threshold Path With Limited Heavy Edges
pub struct Solution;
impl Solution {
    fn can_reach_within_k_heavy(
        graph: &[Vec<(usize, i32)>],
        source: usize,
        target: usize,
        k: i32,
        threshold: i32,
    ) -> bool {
        use std::collections::VecDeque;
        let n = graph.len();
        let mut heavy_cnt = vec![i32::MAX; n];
        let mut deque = VecDeque::new();

        heavy_cnt[source] = 0;
        deque.push_front(source);

        while let Some(u) = deque.pop_front() {
            let cur = heavy_cnt[u];
            if cur > k {
                continue;
            }
            if u == target {
                return true;
            }

            for &(v, w) in &graph[u] {
                let add = if w > threshold { 1 } else { 0 };
                let next = cur + add;
                if next < heavy_cnt[v] && next <= k {
                    heavy_cnt[v] = next;
                    if add == 0 {
                        deque.push_front(v);
                    } else {
                        deque.push_back(v);
                    }
                }
            }
        }

        heavy_cnt[target] <= k
    }

    pub fn minimum_threshold(n: i32, edges: Vec<Vec<i32>>, source: i32, target: i32, k: i32) -> i32 {
        let n = n as usize;
        let source = source as usize;
        let target = target as usize;

        if source == target {
            return 0;
        }

        let mut graph: Vec<Vec<(usize, i32)>> = vec![Vec::new(); n];
        let mut max_w = 0i32;

        for e in &edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            let w = e[2];
            graph[u].push((v, w));
            graph[v].push((u, w));
            max_w = max_w.max(w);
        }

        if !Self::can_reach_within_k_heavy(&graph, source, target, k, max_w) {
            return -1;
        }

        let mut lo = 0i32;
        let mut hi = max_w;
        while lo < hi {
            let mid = lo + ((hi - lo) >> 1);
            if Self::can_reach_within_k_heavy(&graph, source, target, k, mid) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }

        lo
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_minimum_threshold() {
        assert_eq!(
            Solution::minimum_threshold(
                6,
                vec_vec![[0, 1, 5], [1, 2, 3], [3, 4, 4], [4, 5, 1], [1, 4, 2]],
                0,
                3,
                1
            ),
            4
        );
        assert_eq!(
            Solution::minimum_threshold(6, vec_vec![[0, 1, 3], [1, 2, 4], [3, 4, 5], [4, 5, 6]], 0, 4, 1),
            -1
        );
        assert_eq!(
            Solution::minimum_threshold(4, vec_vec![[0, 1, 2], [1, 2, 2], [2, 3, 2], [3, 0, 2]], 0, 0, 0),
            0
        );
    }
}
