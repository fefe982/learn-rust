// https://leetcode.com/problems/minimum-time-for-k-connected-components/
// 3608. Minimum Time for K Connected Components
pub struct Solution;
impl Solution {
    pub fn min_time(n: i32, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = n as usize;
        let mut uf = (0..n).collect();
        let mut cnt = n;
        let mut edges = edges;
        edges.sort_unstable_by_key(|e| std::cmp::Reverse(e[2]));
        fn find(uf: &mut Vec<usize>, x: usize) -> usize {
            if uf[x] != x {
                uf[x] = find(uf, uf[x]);
            }
            uf[x]
        }
        fn union(uf: &mut Vec<usize>, x: usize, y: usize) -> bool {
            let x = find(uf, x);
            let y = find(uf, y);
            if x == y {
                return false;
            } else {
                uf[x] = y;
                return true;
            }
        }
        let k = k as usize;
        for edge in edges {
            if union(&mut uf, edge[0] as usize, edge[1] as usize) {
                cnt -= 1;
            }
            if cnt < k {
                return edge[2];
            }
        }
        0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn min_time() {
        assert_eq!(Solution::min_time(2, vec_vec![[0, 1, 3]], 2), 3);
        assert_eq!(Solution::min_time(3, vec_vec![[0, 1, 2], [1, 2, 4]], 3), 4);
        assert_eq!(Solution::min_time(3, vec_vec![[0, 2, 5]], 2), 0);
    }
}
