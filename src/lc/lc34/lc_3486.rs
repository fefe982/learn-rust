// https://leetcode.com/problems/longest-special-path-ii/
// 3486. Longest Special Path II
pub struct Solution;
impl Solution {
    fn walk(
        g: &Vec<Vec<(usize, i32)>>,
        nums: &Vec<i32>,
        u: usize,
        p: usize,
        lvl: usize,
        start: usize,
        sum: &mut Vec<i32>,
        pos: &mut std::collections::HashMap<i32, usize>,
        dup_pos: usize,
    ) -> (i32, usize) {
        let mut lmax = sum[lvl] - sum[start];
        let mut lmax_len = lvl - start + 1;
        for &(v, w) in &g[u] {
            if v == p {
                continue;
            }
            let cmax;
            let cmax_len;
            sum.push(sum[lvl] + w);
            let num = nums[v];
            if let Some(&pdup) = pos.get(&num) {
                pos.insert(num, lvl + 1);
                if dup_pos == usize::MAX {
                    (cmax, cmax_len) = Self::walk(g, nums, v, u, lvl + 1, start, sum, pos, pdup);
                } else if dup_pos < pdup {
                    (cmax, cmax_len) = Self::walk(g, nums, v, u, lvl + 1, dup_pos + 1, sum, pos, pdup);
                } else {
                    (cmax, cmax_len) = Self::walk(g, nums, v, u, lvl + 1, start.max(pdup + 1), sum, pos, dup_pos);
                }
                pos.insert(num, pdup);
            } else {
                pos.insert(num, lvl + 1);
                (cmax, cmax_len) = Self::walk(g, nums, v, u, lvl + 1, start, sum, pos, dup_pos);
                pos.remove(&num);
            }
            sum.pop();
            if cmax > lmax {
                lmax = cmax;
                lmax_len = cmax_len;
            } else if cmax == lmax && cmax_len < lmax_len {
                lmax_len = cmax_len;
            }
        }
        (lmax, lmax_len)
    }
    pub fn longest_special_path(edges: Vec<Vec<i32>>, nums: Vec<i32>) -> Vec<i32> {
        let n = edges.len() + 1;
        let mut g = vec![vec![]; n];
        for edge in edges {
            g[edge[0] as usize].push((edge[1] as usize, edge[2]));
            g[edge[1] as usize].push((edge[0] as usize, edge[2]));
        }
        let mut pos = std::collections::HashMap::new();
        pos.insert(nums[0], 0);
        let (lmax, lmax_len) = Self::walk(&g, &nums, 0, usize::MAX, 0, 0, &mut vec![0], &mut pos, usize::MAX);
        vec![lmax, lmax_len as i32]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn longest_special_path() {
        assert_eq!(
            Solution::longest_special_path(
                vec_vec![
                    [0, 1, 1],
                    [1, 2, 3],
                    [1, 3, 1],
                    [2, 4, 6],
                    [4, 7, 2],
                    [3, 5, 2],
                    [3, 6, 5],
                    [6, 8, 3]
                ],
                vec![1, 1, 0, 3, 1, 2, 1, 1, 0]
            ),
            vec![9, 3]
        );
        assert_eq!(
            Solution::longest_special_path(vec_vec![[1, 0, 3], [0, 2, 4], [0, 3, 5]], vec![1, 1, 0, 2]),
            vec![5, 2]
        );
    }
}
