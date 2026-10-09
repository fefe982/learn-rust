// https://leetcode.com/problems/maximum-weighted-k-edge-path/
// 3543. Maximum Weighted K-Edge Path
pub struct Solution;
impl Solution {
    pub fn max_weight(n: i32, edges: Vec<Vec<i32>>, k: i32, t: i32) -> i32 {
        let n = n as usize;
        let mut wset = vec![std::collections::HashSet::new(); n];
        for set in &mut wset {
            set.insert(0);
        }
        for _ in 0..k {
            let mut nwset = vec![std::collections::HashSet::new(); n];
            for edge in &edges {
                let (u, v, we) = (edge[0] as usize, edge[1] as usize, edge[2]);
                for &w in &wset[u] {
                    let nw = w + we;
                    if nw < t {
                        nwset[v].insert(nw);
                    }
                }
            }
            wset = nwset;
        }
        let mut ans = -1;
        for set in wset {
            for w in set {
                ans = ans.max(w);
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
    fn max_weight() {
        assert_eq!(Solution::max_weight(3, vec_vec![[0, 1, 1], [1, 2, 2]], 2, 4), 3);
        assert_eq!(Solution::max_weight(3, vec_vec![[0, 1, 2], [0, 2, 3]], 1, 3), 2);
        assert_eq!(Solution::max_weight(3, vec_vec![[0, 1, 6], [1, 2, 8]], 1, 6), -1);
    }
}
