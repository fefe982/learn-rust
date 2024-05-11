// https://leetcode.com/problems/longest-common-subpath/
// 1923. Longest Common Subpath
pub struct Solution;
#[derive(Debug, Clone)]
struct State {
    link: usize,
    len: usize,
    transition: std::collections::HashMap<i32, usize>,
}
impl State {
    fn new(len: usize) -> Self {
        Self {
            link: usize::MAX,
            len,
            transition: std::collections::HashMap::new(),
        }
    }
}
impl Solution {
    fn sa_build(v: &Vec<i32>) -> Vec<State> {
        let mut sa = vec![State::new(0)];
        let mut last = 0;
        for &n in v {
            let cur = sa.len();
            sa.push(State::new(sa[last].len + 1));
            let mut p = last;
            loop {
                sa[p].transition.insert(n, cur);
                p = sa[p].link;
                if p == usize::MAX || sa[p].transition.contains_key(&n) {
                    break;
                }
            }
            if p == usize::MAX {
                sa[cur].link = 0;
                sa[0].transition.insert(n, cur);
            } else {
                let &q = sa[p].transition.get(&n).unwrap();
                if sa[q].len == sa[p].len + 1 {
                    sa[cur].link = q;
                } else {
                    let qc = sa.len();
                    sa.push(sa[q].clone());
                    sa[qc].len = sa[p].len + 1;
                    sa[cur].link = qc;
                    sa[q].link = qc;
                    while p != usize::MAX {
                        if let Some(&pn) = sa[p].transition.get(&n) {
                            if pn == q {
                                sa[p].transition.insert(n, qc);
                            }
                        }
                        p = sa[p].link;
                    }
                }
            }
            last = cur;
        }
        sa
    }
    fn sa_search(v: &Vec<i32>, sa: &Vec<State>, anchor: i32) -> (usize, Vec<i32>) {
        let mut idx = 0;
        let mut l = 0;
        let mut maxl = 0;
        let mut nv = vec![];
        for (j, &n) in v.iter().chain(&[-1]).enumerate() {
            if l > 0 && !sa[idx].transition.contains_key(&n) {
                if l > 1 || v[j - 1] != anchor {
                    maxl = maxl.max(l);
                    nv.extend_from_slice(&v[j - l..j]);
                    nv.push(anchor);
                }
            }
            while idx > 0 && !sa[idx].transition.contains_key(&n) {
                idx = sa[idx].link;
                l = sa[idx].len;
            }
            if let Some(&nidx) = sa[idx].transition.get(&n) {
                idx = nidx;
                l += 1;
            }
        }
        (maxl, nv)
    }
    pub fn longest_common_subpath(n: i32, paths: Vec<Vec<i32>>) -> i32 {
        let mut sa = Self::sa_build(&paths[0]);
        let mut maxl = 0;
        for j in 1..paths.len() {
            let (l, nv) = Self::sa_search(&paths[j], &sa, n);
            maxl = l;
            sa = Self::sa_build(&nv);
        }
        maxl as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_longest_common_subpath() {
        assert_eq!(
            Solution::longest_common_subpath(
                10,
                vec_vec![[1, 2, 3, 4, 3, 4, 2, 1], [1, 2, 5, 2, 1], [2, 1, 3, 4, 3, 4, 3, 4]]
            ),
            2
        );
        assert_eq!(
            Solution::longest_common_subpath(
                10,
                vec_vec![
                    [
                        1, 7, 0, 6, 9, 0, 7, 4, 3, 9, 1, 5, 0, 8, 0, 6, 3, 6, 0, 8, 3, 7, 8, 3, 5, 3, 7, 4, 0, 6, 8, 1,
                        4
                    ],
                    [
                        1, 7, 0, 6, 9, 0, 7, 4, 3, 9, 1, 5, 0, 8, 0, 6, 3, 6, 0, 8, 3, 7, 8, 3, 5, 3, 7, 4, 0, 6, 8, 1,
                        5
                    ],
                    [
                        8, 1, 7, 0, 6, 9, 0, 7, 4, 3, 9, 1, 5, 0, 8, 0, 6, 3, 6, 0, 8, 3, 7, 8, 3, 5, 3, 7, 4, 0, 6, 8,
                        1
                    ]
                ]
            ),
            32
        );
        assert_eq!(
            Solution::longest_common_subpath(5, vec_vec![[0, 1, 2, 3, 4], [2, 3, 4], [4, 0, 1, 2, 3]]),
            2
        );
        assert_eq!(Solution::longest_common_subpath(3, vec_vec![[0], [1], [2]]), 0);
        assert_eq!(
            Solution::longest_common_subpath(5, vec_vec![[0, 1, 2, 3, 4], [4, 3, 2, 1, 0]]),
            1
        );
    }
}
