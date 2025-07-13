// https://leetcode.com/problems/sort-items-by-groups-respecting-dependencies/
// 1203. Sort Items by Groups Respecting Dependencies
pub struct Solution;
impl Solution {
    pub fn sort_items(n: i32, m: i32, group: Vec<i32>, before_items: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let m = m as usize;
        let mut g = vec![std::collections::HashSet::<usize>::new(); n + m * 2];
        let mut f = g.clone();
        for (i, &ig) in group.iter().enumerate() {
            if ig >= 0 {
                g[i].insert(n + ig as usize * 2);
                f[n + ig as usize * 2].insert(i);
                g[n + ig as usize * 2 + 1].insert(i);
                f[i].insert(n + ig as usize * 2 + 1);
            }
        }
        for (i, biv) in before_items.into_iter().enumerate() {
            for bi in biv {
                let bi = bi as usize;
                if group[i] == group[bi] && group[i] != -1 {
                    g[i].insert(bi);
                    f[bi].insert(i);
                } else {
                    let gi = if group[i] == -1 {
                        i
                    } else {
                        n + group[i] as usize * 2
                    };
                    let gbi = if group[bi] == -1 {
                        bi
                    } else {
                        n + group[bi] as usize * 2 + 1
                    };

                    g[gi].insert(gbi);
                    f[gbi].insert(gi);
                }
            }
        }
        let mut q = vec![];
        for (i, ig) in g.iter().enumerate() {
            if ig.is_empty() {
                q.push(i);
            }
        }
        let mut res = vec![];
        while let Some(i) = q.pop() {
            if i < n {
                res.push(i as i32);
            }
            for &fi in &f[i] {
                g[fi].remove(&i);
                if g[fi].is_empty() {
                    q.push(fi);
                }
            }
        }
        if res.len() == n {
            res
        } else {
            vec![]
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn sort_items() {
        assert_eq!(
            Solution::sort_items(
                8,
                2,
                vec![-1, -1, 1, 0, 0, 1, 0, -1],
                vec_vec![[], [6], [5], [6], [3, 6], [], [], []]
            ),
            vec![5, 2, 6, 3, 4, 1, 7, 0]
        );
        assert_eq!(
            Solution::sort_items(
                8,
                2,
                vec![-1, -1, 1, 0, 0, 1, 0, -1],
                vec_vec![[], [6], [5], [6], [3], [], [4], []]
            ),
            vec![]
        );
    }
}
