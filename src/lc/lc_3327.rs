// https://leetcode.com/problems/check-if-dfs-strings-are-palindromes/
// 3327. Check if Two String Arrays are Equivalent
pub struct Solution;
const MOD: i64 = 1_000_000_007;
impl Solution {
    fn dfs(tree: &Vec<Vec<usize>>, s: &[u8], n: usize, pow: &Vec<i64>, res: &mut Vec<bool>) -> (usize, i64, i64) {
        let mut len = 0;
        let mut fwd = 0;
        let mut bwd = 0;
        for &c in &tree[n] {
            let (clen, cfwd, cbwd) = Self::dfs(tree, s, c, pow, res);
            fwd = (fwd * pow[clen] + cfwd) % MOD;
            bwd = (bwd + cbwd * pow[len]) % MOD;
            len += clen;
        }
        let c = (s[n] - b'a') as i64;
        fwd = (fwd * pow[1] + c) % MOD;
        bwd = (bwd + c * pow[len]) % MOD;
        len += 1;
        if fwd == bwd {
            res[n] = true;
        }
        (len, fwd, bwd)
    }
    pub fn find_answer(parent: Vec<i32>, s: String) -> Vec<bool> {
        let len = parent.len();
        if len == 1 {
            return vec![true];
        }
        let mut tree = vec![vec![]; len];
        let mut res = vec![false; len];
        let mut pow = vec![1; len];
        for (n, p) in parent.into_iter().enumerate().skip(1) {
            tree[p as usize].push(n);
        }
        for i in 1..len {
            pow[i] = (pow[i - 1] * 26) % MOD;
        }
        Self::dfs(&tree, s.as_bytes(), 0, &pow, &mut res);
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_answer() {
        assert_eq!(Solution::find_answer(vec![-1], "d".to_string()), [true]);
        assert_eq!(
            Solution::find_answer(vec![-1, 0, 0, 1, 1, 2], "aababa".to_string()),
            [true, true, false, true, true, true]
        );
        assert_eq!(
            Solution::find_answer(vec![-1, 0, 0, 0, 0], "aabcb".to_string()),
            [true, true, true, true, true]
        );
    }
}
