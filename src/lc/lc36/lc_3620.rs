// https://leetcode.com/problems/network-recovery-pathways/
// 3620. Network Recovery Pathways
pub struct Solution;
impl Solution {
    pub fn find_max_path_score(edges: Vec<Vec<i32>>, online: Vec<bool>, k: i64) -> i32 {
        let n = online.len();
        let mut deg_o = vec![0; n];
        let mut deg_i = vec![0; n];
        let mut g = vec![vec![]; n];
        let mut gr = vec![vec![]; n];
        for e in &edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            let w = e[2];
            if v == 0 || u == n - 1 || !online[u] || !online[v] {
                continue;
            }
            g[u].push((v, w));
            gr[v].push(u);
            deg_o[u] += 1;
            deg_i[v] += 1;
        }
        let mut q = std::collections::VecDeque::new();
        for i in 1..n - 1 {
            if deg_i[i] == 0 {
                q.push_back(i);
            }
        }
        while let Some(u) = q.pop_front() {
            for &(v, _) in &g[u] {
                deg_i[v] -= 1;
                if deg_i[v] == 0 {
                    q.push_back(v);
                }
            }
        }
        for i in 0..n - 2 {
            if deg_o[i] == 0 {
                q.push_back(i);
            }
        }
        while let Some(u) = q.pop_front() {
            for &v in &gr[u] {
                deg_o[v] -= 1;
                if deg_o[v] == 0 {
                    q.push_back(v);
                }
            }
        }
        let mut v = Vec::with_capacity(n);
        v.push(0);
        let mut i = 0;
        let mut upper = 0;
        while i < v.len() {
            let u = v[i];
            let mut j = 0;
            for k in 0..g[u].len() {
                let (to, w) = g[u][k];
                if to != n - 1 && deg_o[to] == 0 {
                    continue;
                }
                upper = upper.max(w);
                if j != k {
                    g[u][j] = g[u][k];
                }
                deg_i[to] -= 1;
                if deg_i[to] == 0 {
                    v.push(to);
                }
                j += 1;
            }
            g[u].truncate(j);
            i += 1;
        }
        if v[v.len() - 1] != n - 1 {
            return -1;
        }
        v.pop();
        let check = |limit: i32| -> bool {
            let mut dp = vec![i64::MAX; n];
            dp[0] = 0;
            for &u in &v {
                if dp[u] == i64::MAX {
                    continue;
                }
                for &(to, w) in &g[u] {
                    if w >= limit {
                        dp[to] = dp[to].min(dp[u] + w as i64);
                    }
                }
            }
            dp[n - 1] <= k
        };
        if check(upper) {
            return upper;
        }
        if !check(0) {
            return -1;
        }
        let mut lower = 0;
        while lower + 1 < upper {
            let mid = (lower + upper) / 2;
            if check(mid) {
                lower = mid;
            } else {
                upper = mid;
            }
        }
        lower
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn find_max_path_score() {
        assert_eq!(
            Solution::find_max_path_score(vec_vec![[0, 1, 8]], vec![true, true], 11),
            8
        );
        assert_eq!(
            Solution::find_max_path_score(
                vec_vec![[0, 1, 5], [1, 3, 10], [0, 2, 3], [2, 3, 4]],
                vec![true, true, true, true],
                10
            ),
            3
        );
        assert_eq!(
            Solution::find_max_path_score(
                vec_vec![[0, 1, 7], [1, 4, 5], [0, 2, 6], [2, 3, 6], [3, 4, 2], [2, 4, 6]],
                vec![true, true, true, false, true],
                12
            ),
            6
        );
    }
}
