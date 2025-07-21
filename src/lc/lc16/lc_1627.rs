// https://leetcode.com/problems/graph-connectivity-with-threshold/
// 1627. Graph Connectivity With Threshold
pub struct Solution;
impl Solution {
    fn get_p(p: &mut Vec<usize>, x: usize) -> usize {
        let mut px = x;
        while p[px] != px {
            px = p[px];
        }
        p[x] = px;
        px
    }
    fn merge(p: &mut Vec<usize>, i: usize, j: usize) {
        let pi = Self::get_p(p, i);
        let pj = Self::get_p(p, j);
        p[pi] = pj;
    }
    pub fn are_connected(n: i32, threshold: i32, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let n = n as usize;
        let threshold = threshold as usize;
        let mut p = (0..=n).collect::<Vec<_>>();
        for i in threshold * 2..=n {
            let mut j = 1;
            while j * j <= i {
                if i % j == 0 {
                    if j > threshold {
                        Self::merge(&mut p, i, j);
                    }
                    if j != 1 && i / j != j && i / j > threshold {
                        Self::merge(&mut p, i, i / j);
                    }
                }
                j += 1;
            }
        }
        queries
            .into_iter()
            .map(|v| Self::get_p(&mut p, v[0] as usize) == Self::get_p(&mut p, v[1] as usize))
            .collect::<Vec<_>>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_are_connected() {
        assert_eq!(
            Solution::are_connected(6, 2, vec_vec![[1, 4], [2, 5], [3, 6]]),
            vec![false, false, true]
        );
        assert_eq!(
            Solution::are_connected(6, 0, vec_vec![[4, 5], [3, 4], [3, 2], [2, 6], [1, 3]]),
            vec![true, true, true, true, true]
        );
        assert_eq!(
            Solution::are_connected(5, 1, vec_vec![[4, 5], [4, 5], [3, 2], [2, 3], [3, 4]]),
            vec![false, false, false, false, false]
        );
    }
}
