// https://leetcode.com/problems/second-minimum-time-to-reach-destination/
// 2045. Second Minimum Time to Reach Destination
pub struct Solution;
impl Solution {
    fn dist(g: &Vec<Vec<usize>>, u: usize) -> Vec<i32> {
        let mut v = vec![-1; g.len()];
        let mut q = std::collections::VecDeque::new();
        q.push_back(u);
        v[u] = 0;
        while let Some(c) = q.pop_front() {
            for &n in &g[c] {
                if v[n] == -1 {
                    v[n] = v[c] + 1;
                    q.push_back(n);
                }
            }
        }
        v
    }
    pub fn second_minimum(n: i32, edges: Vec<Vec<i32>>, time: i32, change: i32) -> i32 {
        let mut g = vec![vec![]; n as usize + 1];
        for e in &edges {
            g[e[0] as usize].push(e[1] as usize);
            g[e[1] as usize].push(e[0] as usize);
        }
        let d1 = Self::dist(&g, 1);
        let dn = Self::dist(&g, n as usize);
        let s = d1[n as usize];
        let mut second = i32::MAX;
        for e in edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            let mut d = d1[u] + dn[v] + 1;
            if d > s && d < second {
                second = d;
            }
            d = d1[v] + dn[u] + 1;
            if d > s && d < second {
                second = d;
            }
        }
        assert!(second != i32::MAX);
        let mut seg = 0;
        let mut seg_len = second;
        for i in 1..second {
            let nstart = i * time;
            if nstart / change % 2 == 1 {
                seg_len = i;
                seg = (nstart / change + 1) * change;
                break;
            }
        }
        (second - 1) / seg_len * seg + ((second - 1) % seg_len + 1) * time
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_second_minimum() {
        assert_eq!(
            Solution::second_minimum(5, vec_vec![[1, 2], [1, 3], [1, 4], [3, 4], [4, 5]], 3, 5),
            13
        );
        assert_eq!(Solution::second_minimum(2, vec_vec![[1, 2]], 3, 2), 11);
    }
}
