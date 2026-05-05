// https://leetcode.com/problems/the-time-when-the-network-becomes-idle/
// 2039. The Time When the Network Becomes Idle
pub struct Solution;
impl Solution {
    pub fn network_becomes_idle(edges: Vec<Vec<i32>>, patience: Vec<i32>) -> i32 {
        let n = patience.len();
        let mut g = vec![vec![]; n];
        for e in edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            g[u].push(v);
            g[v].push(u);
        }

        let mut dist = vec![-1_i32; n];
        let mut q = std::collections::VecDeque::new();
        dist[0] = 0;
        q.push_back(0_usize);

        while let Some(u) = q.pop_front() {
            for &v in &g[u] {
                if dist[v] == -1 {
                    dist[v] = dist[u] + 1;
                    q.push_back(v);
                }
            }
        }

        let mut last = 0_i32;
        for i in 1..n {
            let round_trip = dist[i] * 2;
            let p = patience[i];
            let last_send = (round_trip - 1) / p * p;
            let last_reply = last_send + round_trip;
            last = last.max(last_reply);
        }

        last + 1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn network_becomes_idle() {
        assert_eq!(
            Solution::network_becomes_idle(vec_vec![[0, 1], [1, 2]], vec![0, 2, 1]),
            8
        );
        assert_eq!(
            Solution::network_becomes_idle(vec_vec![[0, 1], [0, 2], [1, 2]], vec![0, 10, 10]),
            3
        );
    }
}
