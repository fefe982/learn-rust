// https://leetcode.com/problems/palindromic-path-queries-in-a-tree/
// 3841. Palindromic Path Queries in a Tree
pub struct Solution;
impl Solution {
    fn walk(
        g: &Vec<Vec<usize>>,
        s: &Vec<i32>,
        iin: &mut Vec<usize>,
        iout: &mut Vec<usize>,
        xor: &mut Vec<i32>,
        pa: &mut Vec<Vec<usize>>,
        lvl: &mut Vec<i32>,
        u: usize,
        p: usize,
        idx: &mut usize,
        xor_p: i32,
        l: i32,
    ) {
        lvl[u] = l;
        iin[u] = *idx;
        let xor_u = xor_p ^ s[u];
        xor[*idx] = xor_u;
        if p != usize::MAX {
            pa[u][0] = p;
            for i in 1..pa[p].len() {
                pa[u][i] = pa[pa[u][i - 1]][i - 1];
            }
        }
        for &v in &g[u] {
            if v != p {
                *idx += 1;
                Self::walk(g, s, iin, iout, xor, pa, lvl, v, u, idx, xor_u, l + 1);
            }
        }
        iout[u] = *idx;
    }
    fn tree_xor(tree: &mut Vec<i32>, idx: usize, val: i32) {
        let mut idx = idx;
        while idx < tree.len() {
            tree[idx] ^= val;
            idx += 1 << idx.trailing_zeros();
        }
    }
    fn tree_query(tree: &Vec<i32>, idx: usize) -> i32 {
        let mut idx = idx;
        let mut ans = 0;
        while idx > 0 {
            ans ^= tree[idx];
            idx -= 1 << idx.trailing_zeros();
        }
        ans
    }
    fn query_pa_k(u: usize, k: i32, pa: &Vec<Vec<usize>>) -> usize {
        let mut u = u;
        let mut k = k;
        while k > 0 {
            u = pa[u][k.trailing_zeros() as usize];
            k = k & (k - 1);
        }
        u
    }
    pub fn palindrome_path(n: i32, edges: Vec<Vec<i32>>, s: String, queries: Vec<String>) -> Vec<bool> {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        let mut s = s.as_bytes().iter().map(|&c| 1 << (c - b'a')).collect::<Vec<_>>();
        for e in edges {
            g[e[0] as usize].push(e[1] as usize);
            g[e[1] as usize].push(e[0] as usize);
        }
        let mut iin = vec![0; n];
        let mut iout = vec![0; n];
        let mut xor = vec![0; n];
        let mut lvl = vec![0; n];
        let lpa = (usize::BITS - n.leading_zeros()) as usize;
        let mut pa = vec![vec![0; lpa]; n];
        Self::walk(
            &g,
            &s,
            &mut iin,
            &mut iout,
            &mut xor,
            &mut pa,
            &mut lvl,
            0,
            usize::MAX,
            &mut 0,
            0,
            0,
        );
        let mut tree = vec![0; n + 2];
        let mut ans = vec![];
        for q in queries {
            let qsplit = q.split(' ').collect::<Vec<_>>();
            if qsplit[0] == "query" {
                let u = qsplit[1].parse::<usize>().unwrap();
                let v = qsplit[2].parse::<usize>().unwrap();
                let lu = lvl[u];
                let lv = lvl[v];
                let mut pu = u;
                let mut pv = v;
                if lu < lv {
                    pv = Self::query_pa_k(pv, lv - lu, &pa);
                } else if lu > lv {
                    pu = Self::query_pa_k(pu, lu - lv, &pa);
                }
                let lca;
                if pu == pv {
                    lca = pu;
                } else {
                    for i in (0..lpa).rev() {
                        if pa[pu][i] != pa[pv][i] {
                            pu = pa[pu][i];
                            pv = pa[pv][i];
                        }
                    }
                    lca = pa[pu][0];
                }
                let qu = Self::tree_query(&tree, iin[u] + 1);
                let qv = Self::tree_query(&tree, iin[v] + 1);
                let val = xor[iin[u]] ^ xor[iin[v]] ^ s[lca] ^ qu ^ qv;
                ans.push(val == 0 || (val & (val - 1)) == 0);
            } else {
                let u = qsplit[1].parse::<usize>().unwrap();
                let c = 1 << (qsplit[2].as_bytes()[0] - b'a');
                let x = c ^ s[u];
                s[u] = c;
                Self::tree_xor(&mut tree, iin[u] + 1, x);
                Self::tree_xor(&mut tree, iout[u] + 2, x);
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn palindrome_path() {
        assert_eq!(
            Solution::palindrome_path(3, vec_vec![[0, 1], [1, 2]], "jfj".to_string(), vec_str!["query 2 1"]),
            [false]
        );
        assert_eq!(
            Solution::palindrome_path(
                3,
                vec_vec![[0, 1], [1, 2]],
                "aac".to_string(),
                vec_str!["query 0 2", "update 1 b", "query 0 2"]
            ),
            [true, false]
        );
        assert_eq!(
            Solution::palindrome_path(
                4,
                vec_vec![[0, 1], [0, 2], [0, 3]],
                "abca".to_string(),
                vec_str!["query 1 2", "update 0 b", "query 2 3", "update 3 a", "query 1 3"]
            ),
            [false, false, true]
        );
    }
}
