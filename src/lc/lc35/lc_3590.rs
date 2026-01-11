// https://leetcode.com/problems/kth-smallest-path-xor-sum/
// 3590. Kth Smallest Path XOR Sum
pub struct Solution;
impl Solution {
    fn insert(cnt: &mut Vec<i32>, lc: &mut Vec<usize>, rc: &mut Vec<usize>, l: usize, r: usize, v: usize) -> usize {
        let idx = cnt.len();
        cnt.push(0);
        lc.push(0);
        rc.push(0);
        if l + 1 == r {
            cnt[idx] = 1;
            return idx;
        }
        let mid = (l + r) >> 1;
        if v < mid {
            lc[idx] = Self::insert(cnt, lc, rc, l, mid, v);
        } else {
            rc[idx] = Self::insert(cnt, lc, rc, mid, r, v);
        }
        cnt[idx] = 1;
        idx
    }
    fn merge(
        cnt: &mut Vec<i32>,
        lc: &mut Vec<usize>,
        rc: &mut Vec<usize>,
        r1: usize,
        r2: usize,
        l: usize,
        r: usize,
    ) -> usize {
        if r1 == 0 || r2 == 0 {
            return r1 | r2;
        }
        if l + 1 == r {
            return r1;
        }
        let mid = (l + r) >> 1;
        lc[r1] = Self::merge(cnt, lc, rc, lc[r1], lc[r2], l, mid);
        rc[r1] = Self::merge(cnt, lc, rc, rc[r1], rc[r2], mid, r);
        cnt[r1] = cnt[lc[r1]] + cnt[rc[r1]];
        r1
    }
    fn kth(cnt: &mut Vec<i32>, lc: &mut Vec<usize>, rc: &mut Vec<usize>, rt: usize, l: usize, r: usize, k: i32) -> i32 {
        if l + 1 == r {
            return l as i32;
        }
        let mid = (l + r) >> 1;
        if cnt[lc[rt]] <= k {
            return Self::kth(cnt, lc, rc, rc[rt], mid, r, k - cnt[lc[rt]]);
        } else {
            return Self::kth(cnt, lc, rc, lc[rt], l, mid, k);
        }
    }
    fn dfs(
        g: &Vec<Vec<usize>>,
        vals: &Vec<i32>,
        rg: usize,
        xor: usize,
        cnt: &mut Vec<i32>,
        lc: &mut Vec<usize>,
        rc: &mut Vec<usize>,
        q: &std::collections::HashMap<usize, Vec<(usize, i32)>>,
        ans: &mut Vec<i32>,
    ) -> usize {
        let x = xor ^ vals[rg] as usize;
        let mut rt = Self::insert(cnt, lc, rc, 0, 1 << 17, x);
        for &c in &g[rg] {
            let ct = Self::dfs(g, vals, c, x, cnt, lc, rc, q, ans);
            rt = Self::merge(cnt, lc, rc, rt, ct, 0, 1 << 17);
        }
        if let Some(qv) = q.get(&rg) {
            for &(iq, k) in qv {
                if k > cnt[rt] {
                    ans[iq] = -1;
                } else {
                    ans[iq] = Self::kth(cnt, lc, rc, rt, 0, 1 << 17, k - 1);
                }
            }
        }
        rt
    }
    pub fn kth_smallest(par: Vec<i32>, vals: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = vals.len();
        let bits = 20;
        let mut cnt = Vec::with_capacity(n * bits);
        let mut lc = Vec::with_capacity(n * bits);
        let mut rc = Vec::with_capacity(n * bits);
        cnt.push(0);
        lc.push(0);
        rc.push(0);
        let mut q = std::collections::HashMap::new();
        let mut ans = vec![0; queries.len()];
        for (iq, query) in queries.into_iter().enumerate() {
            q.entry(query[0] as usize).or_insert(Vec::new()).push((iq, query[1]));
        }
        let mut g = vec![vec![]; n];
        for (i, p) in par.into_iter().enumerate() {
            if p >= 0 {
                g[p as usize].push(i);
            }
        }
        Self::dfs(&g, &vals, 0, 0, &mut cnt, &mut lc, &mut rc, &q, &mut ans);
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn kth_smallest() {
        assert_eq!(
            Solution::kth_smallest(vec![-1, 0, 0], vec![1, 1, 1], vec_vec![[0, 1], [0, 2], [0, 3]]),
            [0, 1, -1]
        );
        assert_eq!(
            Solution::kth_smallest(vec![-1, 0, 1], vec![5, 2, 7], vec_vec![[0, 1], [1, 2], [1, 3], [2, 1]]),
            [0, 7, -1, 0]
        );
    }
}
