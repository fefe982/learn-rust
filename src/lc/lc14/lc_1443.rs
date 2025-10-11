// https://leetcode.com/problems/minimum-time-to-collect-all-apples-in-a-tree/
// 1443. Minimum Time to Collect All Apples in a Tree
pub struct Solution;
impl Solution {
    fn walk(edges: &Vec<Vec<usize>>, has_apple: &Vec<bool>, u: usize, p: usize) -> i32 {
        let mut time = 0;
        for &v in &edges[u] {
            let v = v as usize;
            if v != p {
                time += Self::walk(edges, has_apple, v, u);
            }
        }
        if time > 0 || has_apple[u] {
            time += 2;
        }
        time
    }
    pub fn min_time(n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
        let mut p = vec![vec![]; n as usize];
        for e in edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            p[u].push(v);
            p[v].push(u);
        }
        (Self::walk(&p, &has_apple, 0, n as usize) - 2).max(0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn min_time() {
        assert_eq!(
            Solution::min_time(
                7,
                vec_vec![[0, 1], [0, 2], [1, 4], [1, 5], [2, 3], [2, 6]],
                vec![false, false, true, false, true, true, false]
            ),
            8
        );
        assert_eq!(
            Solution::min_time(
                7,
                vec_vec![[0, 1], [0, 2], [1, 4], [1, 5], [2, 3], [2, 6]],
                vec![false, false, true, false, false, true, false]
            ),
            6
        );
        assert_eq!(
            Solution::min_time(
                7,
                vec_vec![[0, 1], [0, 2], [1, 4], [1, 5], [2, 3], [2, 6]],
                vec![false, false, false, false, false, false, false]
            ),
            0
        );
    }
}
