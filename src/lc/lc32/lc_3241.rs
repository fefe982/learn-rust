// https://leetcode.com/problems/time-taken-to-mark-all-nodes/
// 3241. Time Taken to Mark All Nodes
pub struct Solution;
impl Solution {
    fn exchng(
        g: &Vec<Vec<usize>>,
        max_child: &mut Vec<(i32, i32)>,
        maxp: (i32, i32),
        p: usize,
        n: usize,
        ans: &mut Vec<i32>,
    ) {
        let mut maxn = max_child[n];
        let nc = maxn.0 + 2 - n as i32 % 2;
        let pc = if nc == maxp.0 { maxp.1 } else { maxp.0 } + 2 - (p % 2) as i32;
        if pc > maxn.1 {
            maxn.1 = maxn.0;
            if pc > maxn.0 {
                maxn.0 = pc;
            } else {
                maxn.1 = pc;
            }
        }
        ans[n] = maxn.0;
        for &c in &g[n] {
            if c != p {
                Self::exchng(g, max_child, maxn, n, c, ans);
            }
        }
    }
    fn build_tree(g: &Vec<Vec<usize>>, max_child: &mut Vec<(i32, i32)>, p: usize, n: usize) -> i32 {
        let mut max = (0, 0);
        for &c in &g[n] {
            if c != p {
                let v = Self::build_tree(g, max_child, n, c);
                if v > max.1 {
                    max.1 = max.0;
                    if v > max.0 {
                        max.0 = v;
                    } else {
                        max.1 = v;
                    }
                }
            }
        }
        max_child[n] = max;
        max.0 + 2 - n as i32 % 2
    }
    pub fn time_taken(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let len = edges.len() + 1;
        let mut g = vec![vec![]; len];
        for e in edges {
            g[e[0] as usize].push(e[1] as usize);
            g[e[1] as usize].push(e[0] as usize);
        }
        let mut ans = vec![0; len];
        let mut max_child = vec![(0, 0); len];
        let max0 = Self::build_tree(&g, &mut max_child, usize::MAX, 0);
        Self::exchng(&g, &mut max_child, (max0, -1), usize::MAX, 0, &mut ans);
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_time_taken() {
        assert_eq!(Solution::time_taken(vec_vec![[0, 1], [0, 2]]), [2, 4, 3]);
        assert_eq!(Solution::time_taken(vec_vec![[0, 1]]), [1, 2]);
        assert_eq!(
            Solution::time_taken(vec_vec![[2, 4], [0, 1], [2, 3], [0, 2]]),
            [4, 6, 3, 5, 5]
        );
    }
}
