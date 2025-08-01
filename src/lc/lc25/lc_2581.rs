// https://leetcode.com/problems/count-number-of-possible-root-nodes/
// 2581. Count Number of Possible Root Nodes
pub struct Solution;
impl Solution {
    fn swap(
        g: &Vec<Vec<usize>>,
        guesses: &std::collections::HashSet<(usize, usize)>,
        p: usize,
        c: usize,
        cnt: usize,
        k: usize,
    ) -> i32 {
        let mut tot = 0;
        for &n in &g[c] {
            if n == p {
                continue;
            }
            let mut ncnt = cnt;
            if guesses.contains(&(n, c)) {
                ncnt += 1;
            }
            if guesses.contains(&(c, n)) {
                ncnt -= 1;
            }
            if ncnt >= k {
                tot += 1;
            }
            tot += Self::swap(g, guesses, c, n, ncnt, k);
        }
        tot
    }
    pub fn root_count(edges: Vec<Vec<i32>>, guesses: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize;
        let mut g = vec![vec![]; edges.len() + 1];
        for e in edges {
            g[e[0] as usize].push(e[1] as usize);
            g[e[1] as usize].push(e[0] as usize);
        }
        let guesses =
            std::collections::HashSet::<_>::from_iter(guesses.into_iter().map(|x| (x[0] as usize, x[1] as usize)));
        let mut cnt = 0;
        let mut q = vec![(g.len(), 0)];
        while let Some((p, c)) = q.pop() {
            if guesses.contains(&(p, c)) {
                cnt += 1;
            }
            for &n in &g[c] {
                if n != p {
                    q.push((c, n))
                }
            }
        }
        Self::swap(&g, &guesses, g.len(), 0, cnt, k) + if cnt >= k { 1 } else { 0 }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_root_count() {
        assert_eq!(
            Solution::root_count(
                vec_vec![[0, 1], [1, 2], [1, 3], [4, 2]],
                vec_vec![[1, 3], [0, 1], [1, 0], [2, 4]],
                3
            ),
            3
        );
        assert_eq!(
            Solution::root_count(
                vec_vec![[0, 1], [1, 2], [2, 3], [3, 4]],
                vec_vec![[1, 0], [3, 4], [2, 1], [3, 2]],
                1
            ),
            5
        );
    }
}
