// https://leetcode.com/problems/minimize-inversions-in-a-prefix-tree/
// 101016. Minimize Inversions in a Prefix Tree
pub struct Solution;
impl Solution {
    fn dp(cost: &Vec<Vec<i32>>, idx: &Vec<usize>) -> i32 {
        let lim = idx.len();
        if lim == 0 {
            return 0;
        }
        let mut g = vec![vec![0; lim]; 1 << lim];
        let mut f = vec![i32::MAX; 1 << lim];
        f[0] = 0;
        for i in 0..lim {
            f[1 << i] = 0;
        }
        for i in 1..(1 << lim) {
            let mut m: usize = i ^ ((1 << lim) - 1);
            let k = i.trailing_zeros() as usize;
            let ik = i ^ (1 << k);
            while m > 0 {
                let j = m.trailing_zeros() as usize;
                m ^= 1 << j;
                g[i][j] = g[ik][j] + cost[idx[k]][idx[j]];
                let ii = i ^ (1 << j);
                f[ii] = f[ii].min(f[i] + g[i][j]);
            }
        }
        f[(1 << lim) - 1]
    }

    fn dfs(words: &Vec<Vec<char>>, v: &Vec<usize>, d: usize) -> i32 {
        const C: usize = 10;
        let mut ch = vec![vec![]; C];
        let mut cnt = vec![0; C];
        let mut cost = vec![vec![0; C]; C];
        let mut ans = 0;
        for &x in v {
            if words[x].len() == d {
                for cc in 0..C {
                    ans += cnt[cc];
                }
            } else {
                let c = words[x][d] as usize - 'a' as usize;
                ch[c].push(x);
                cnt[c] += 1;
                for cc in 0..C {
                    if c != cc {
                        cost[c][cc] += cnt[cc];
                    }
                }
            }
        }
        let mut idx = vec![];
        for i in 0..C {
            if ch[i].len() > 0 {
                idx.push(i);
            }
        }
        ans += Self::dp(&cost, &idx);
        for x in idx {
            ans += Self::dfs(words, &ch[x], d + 1);
        }
        ans
    }
    pub fn min_inversions(words: Vec<String>, target: Vec<i32>) -> i32 {
        let n = words.len();
        let mut t = Vec::with_capacity(n);
        for i in target {
            t.push(words[i as usize].chars().collect::<Vec<char>>());
        }
        let words = t;
        let v = (0..n).collect::<Vec<usize>>();
        Self::dfs(&words, &v, 0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn min_inversions() {
        assert_eq!(
            Solution::min_inversions(vec_str!["bad", "bag", "fig", "fed"], vec![3, 1, 0, 2]),
            2
        );
        assert_eq!(
            Solution::min_inversions(vec_str!["ab", "ac", "ba", "bc"], vec![0, 3, 1, 2]),
            1
        );
    }
}
