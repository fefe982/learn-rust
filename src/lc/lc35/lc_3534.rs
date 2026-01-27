// https://leetcode.com/problems/path-existence-queries-in-a-graph-ii/
// 3534. Path Existence Queries in a Graph II
pub struct Solution;
impl Solution {
    pub fn path_existence_queries(n: i32, nums: Vec<i32>, max_diff: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut sidx = (0..n).collect::<Vec<usize>>();
        sidx.sort_by_key(|&i| nums[i]);
        let d = (usize::BITS - max_diff.leading_zeros()) as usize;
        let mut rt = vec![vec![0; d]; n];
        let mut j = 0;
        for i in 0..n {
            while j + 1 < n && nums[sidx[j + 1]] - nums[sidx[i]] <= max_diff {
                j += 1;
            }
            rt[sidx[i]][0] = sidx[j];
        }
        for i in 1..d {
            for j in 0..n {
                rt[j][i] = rt[rt[j][i - 1]][i - 1];
            }
        }
        let mut res = Vec::with_capacity(queries.len());
        for q in queries {
            let (mut l, mut r) = (q[0] as usize, q[1] as usize);
            if l == r {
                res.push(0);
                continue;
            }
            if nums[l] > nums[r] {
                std::mem::swap(&mut l, &mut r);
            }
            if nums[rt[l][d - 1]] < nums[r] {
                res.push(-1);
                continue;
            }
            let mut dist = 0;
            for i in (0..d - 1).rev() {
                if nums[rt[l][i]] < nums[r] {
                    l = rt[l][i];
                    dist |= 1 << i;
                }
            }
            res.push(dist + 1);
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn path_existence_queries() {
        assert_eq!(
            Solution::path_existence_queries(5, vec![1, 8, 3, 4, 2], 3, vec_vec![[0, 3], [2, 4]]),
            [1, 1]
        );
        assert_eq!(
            Solution::path_existence_queries(5, vec![5, 3, 1, 9, 10], 2, vec_vec![[0, 1], [0, 2], [2, 3], [4, 3]]),
            [1, 2, -1, 1]
        );
    }
}
