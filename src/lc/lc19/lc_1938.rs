// https://leetcode.com/problems/maximum-genetic-difference-query/
// 1938. Maximum Genetic Difference Query
pub struct Solution;
impl Solution {
    fn add(cnt: &mut Vec<i32>, idx: usize, l: usize, r: usize, n: usize, val: i32) {
        if l + 1 < r {
            let mid = (l + r) / 2;
            if n < mid {
                Self::add(cnt, idx * 2, l, mid, n, val);
            } else {
                Self::add(cnt, idx * 2 + 1, mid, r, n, val);
            }
        }
        cnt[idx] += val;
    }
    fn query(cnt: &Vec<i32>, idx: usize, l: usize, r: usize, n: usize) -> i32 {
        if l + 1 == r {
            return (l ^ n) as i32;
        }
        let mid = (l + r) / 2;
        if (l ^ n < mid ^ n && cnt[idx * 2 + 1] > 0) || cnt[idx * 2] == 0 {
            Self::query(cnt, idx * 2 + 1, mid, r, n)
        } else {
            Self::query(cnt, idx * 2, l, mid, n)
        }
    }
    fn dfs(
        g: &Vec<Vec<usize>>,
        cnt: &mut Vec<i32>,
        sz: usize,
        node: usize,
        q: &Vec<Vec<(usize, i32)>>,
        ans: &mut Vec<i32>,
    ) {
        Self::add(cnt, 1, 0, sz, node, 1);
        for &(iq, v) in q[node].iter() {
            ans[iq] = Self::query(cnt, 1, 0, sz, v as usize);
        }
        for &child in g[node].iter() {
            Self::dfs(g, cnt, sz, child, q, ans);
        }
        Self::add(cnt, 1, 0, sz, node, -1);
    }
    pub fn max_genetic_difference(parents: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let len = parents.len();
        let mut g = vec![vec![]; len];
        let mut root = 0;
        for (i, p) in parents.into_iter().enumerate() {
            if p < 0 {
                root = i;
            } else {
                g[p as usize].push(i);
            }
        }
        let mut q = vec![vec![]; len];
        let mut ans = vec![0; queries.len()];
        for (i, query) in queries.into_iter().enumerate() {
            q[query[0] as usize].push((i, query[1]));
        }
        let mut sz = 1;
        while sz < len {
            sz <<= 1;
        }
        let mut cnt = vec![0; sz * 2];
        Self::dfs(&g, &mut cnt, sz, root, &q, &mut ans);
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_max_genetic_difference() {
        assert_eq!(
            Solution::max_genetic_difference(vec![-1, 0, 1, 1], vec_vec![[0, 2], [3, 2], [2, 5]]),
            [2, 3, 7]
        );
        assert_eq!(
            Solution::max_genetic_difference(vec![3, 7, -1, 2, 0, 7, 0, 2], vec_vec![[4, 6], [1, 15], [0, 5]]),
            [6, 14, 7]
        );
    }
}
