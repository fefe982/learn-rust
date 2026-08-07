// https://leetcode.com/problems/maximum-score-after-applying-operations-on-a-tree/
// 2925. Maximum Score After Applying Operations on a Tree
pub struct Solution;
impl Solution {
    fn walk(g: &Vec<Vec<usize>>, values: &Vec<i32>, u: usize, fa: usize) -> (i64, i64) {
        let mut sumz_take = values[u] as i64;
        let mut sumz_skip = 0;
        let mut c = 0;
        for &v in &g[u] {
            if v == fa {
                continue;
            }
            c += 1;
            let (s_skipped, s_no_skip) = Self::walk(g, values, v, u);
            sumz_take += s_no_skip;
            sumz_skip += s_skipped;
        }
        if c == 0 {
            sumz_take = 0;
        }
        (sumz_skip + values[u] as i64, sumz_take.max(sumz_skip))
    }
    pub fn maximum_score_after_operations(edges: Vec<Vec<i32>>, values: Vec<i32>) -> i64 {
        let n = edges.len() + 1;
        let mut g = vec![vec![]; n];
        for e in edges {
            g[e[0] as usize].push(e[1] as usize);
            g[e[1] as usize].push(e[0] as usize);
        }
        Self::walk(&g, &values, 0, 0).1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn maximum_score_after_operations() {
        assert_eq!(
            Solution::maximum_score_after_operations(
                vec_vec![[0, 1], [0, 2], [0, 3], [2, 4], [4, 5]],
                vec![5, 2, 5, 2, 1, 1]
            ),
            11
        );
        assert_eq!(
            Solution::maximum_score_after_operations(
                vec_vec![[0, 1], [0, 2], [1, 3], [1, 4], [2, 5], [2, 6]],
                vec![20, 10, 9, 7, 4, 3, 5]
            ),
            40
        );
    }
}
