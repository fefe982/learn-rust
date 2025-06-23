// https://leetcode.com/problems/longest-path-with-different-adjacent-characters/
// 2246. Longest Path With Different Adjacent Characters
pub struct Solution;
impl Solution {
    fn traverse(g: &Vec<Vec<usize>>, s: &[u8], u: usize, p: usize, max: &mut i32) -> i32 {
        let mut mlen = vec![0, 0];
        for &n in &g[u] {
            if n == p {
                continue;
            }
            let len = Self::traverse(g, s, n, u, max);
            if s[n] != s[u] {
                if mlen[0] < len {
                    mlen[1] = mlen[0];
                    mlen[0] = len;
                } else if mlen[1] < len {
                    mlen[1] = len;
                }
            }
            *max = (*max).max(mlen[0] + mlen[1] + 1);
        }
        mlen[0] + 1
    }
    pub fn longest_path(parent: Vec<i32>, s: String) -> i32 {
        let mut g = vec![vec![]; parent.len()];
        for i in 1..parent.len() {
            g[i].push(parent[i] as usize);
            g[parent[i] as usize].push(i);
        }
        let s = s.as_bytes();
        let mut max = 1;
        Self::traverse(&g, s, 0, usize::MAX, &mut max);
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_longest_path() {
        assert_eq!(Solution::longest_path(vec![-1], "z".to_string()), 1);
        assert_eq!(Solution::longest_path(vec![-1, 0, 0, 1, 1, 2], "abacbe".to_string()), 3);
        assert_eq!(Solution::longest_path(vec![-1, 0, 0, 0], "aabc".to_string()), 3);
    }
}
