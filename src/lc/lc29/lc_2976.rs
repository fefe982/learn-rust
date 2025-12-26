// https://leetcode.com/problems/minimum-cost-to-convert-string-i/
// 2976. Minimum Cost to Convert String I
pub struct Solution;
impl Solution {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<char>,
        changed: Vec<char>,
        cost: Vec<i32>,
    ) -> i64 {
        let inf = i64::MAX / 2;
        let mut cst = vec![vec![inf; 26]; 26];
        for i in 0..26 {
            cst[i][i] = 0;
        }
        for ((o, t), c) in original.into_iter().zip(changed).zip(cost) {
            let io = (o as u8 - b'a') as usize;
            let it = (t as u8 - b'a') as usize;
            cst[io][it] = cst[io][it].min(c as i64);
        }
        for k in 0..26 {
            for i in 0..26 {
                for j in 0..26 {
                    cst[i][j] = cst[i][j].min(cst[i][k] + cst[k][j]);
                }
            }
        }
        let mut total = 0;
        for (s, c) in source.as_bytes().iter().zip(target.as_bytes().iter()) {
            let is = (s - b'a') as usize;
            let ic = (c - b'a') as usize;
            if cst[is][ic] >= inf {
                return -1;
            }
            total += cst[is][ic];
        }
        total
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_minimum_cost() {
        assert_eq!(
            Solution::minimum_cost(
                "abcd".to_string(),
                "acbe".to_string(),
                vec_chr!["a", "b", "c", "c", "e", "d"],
                vec_chr!["b", "c", "b", "e", "b", "e"],
                vec![2, 5, 5, 1, 2, 20],
            ),
            28
        );
        assert_eq!(
            Solution::minimum_cost(
                "aaaa".to_string(),
                "bbbb".to_string(),
                vec_chr!["a", "c"],
                vec_chr!["c", "b"],
                vec![1, 2],
            ),
            12
        );
        assert_eq!(
            Solution::minimum_cost(
                "abcd".to_string(),
                "abce".to_string(),
                vec_chr!["a"],
                vec_chr!["e"],
                vec![10000],
            ),
            -1
        );
    }
}
