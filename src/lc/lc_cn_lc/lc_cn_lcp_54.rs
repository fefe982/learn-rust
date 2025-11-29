// https://leetcode.cn/problems/s5kipK/
// LCP 54. 夺回据点
pub struct Solution;
impl Solution {
    fn dfs(
        n: usize,
        g: &Vec<Vec<usize>>,
        low: &mut Vec<usize>,
        idx: &mut Vec<usize>,
        i: &mut usize,
        pseg: &mut Vec<Vec<usize>>,
        cut: &mut Vec<bool>,
        stk: &mut Vec<usize>,
    ) {
        low[n] = *i;
        idx[n] = *i;
        *i += 1;
        stk.push(n);
        let mut nc = 0;
        for &c in &g[n] {
            if idx[c] == 0 {
                Self::dfs(c, g, low, idx, i, pseg, cut, stk);
                low[n] = low[n].min(low[c]);
                if low[c] >= idx[n] {
                    nc += 1;
                    if n != 0 || nc > 1 {
                        cut[n] = true;
                    }
                    let mut seg = vec![];
                    while let Some(t) = stk.pop() {
                        seg.push(t);
                        if t == c {
                            break;
                        }
                    }
                    seg.push(n);
                    pseg.push(seg);
                }
            } else {
                low[n] = low[n].min(idx[c]);
            }
        }
    }
    pub fn minimum_cost(cost: Vec<i32>, roads: Vec<Vec<i32>>) -> i64 {
        let n = cost.len();
        let mut g = vec![vec![]; n];
        for r in roads {
            let u = r[0] as usize;
            let v = r[1] as usize;
            g[u].push(v);
            g[v].push(u);
        }
        let mut pseg = vec![];
        let mut cut = vec![false; n];
        Self::dfs(
            0,
            &g,
            &mut vec![0; n],
            &mut vec![0; n],
            &mut 1,
            &mut pseg,
            &mut cut,
            &mut vec![],
        );
        if pseg.len() == 1 {
            return cost.into_iter().min().unwrap() as i64;
        }
        let mut ans = 0;
        let mut max = 0;
        for seg in pseg {
            let mut min = i32::MAX;
            let mut ccut = 0;
            for &n in &seg {
                if cut[n] {
                    ccut += 1;
                } else {
                    min = min.min(cost[n]);
                }
            }
            if ccut == 1 {
                max = max.max(min);
                ans += min as i64;
            }
        }
        ans - max as i64
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn minimum_cost() {
        assert_eq!(
            Solution::minimum_cost(
                vec![1, 2, 3, 4, 5, 6],
                vec_vec![[0, 1], [0, 2], [1, 3], [2, 3], [1, 2], [2, 4], [2, 5]]
            ),
            6
        );
        assert_eq!(
            Solution::minimum_cost(vec![3, 2, 1, 4], vec_vec![[0, 2], [2, 3], [3, 1]]),
            2
        );
    }
}
