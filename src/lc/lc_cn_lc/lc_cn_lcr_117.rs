// https://leetcode.cn/problems/H6lPxb/description/
// LCR 117. 相似字符串组
pub struct Solution;
impl Solution {
    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        let mut uf = (0..strs.len()).collect::<Vec<_>>();
        let mut ans = strs.len() as i32;
        fn find(uf: &mut Vec<usize>, x: usize) -> usize {
            if uf[x] != x {
                uf[x] = find(uf, uf[x]);
            }
            uf[x]
        }
        fn check(a: &str, b: &str) -> bool {
            let mut cnt = 0;
            for (c1, c2) in a.chars().zip(b.chars()) {
                if c1 != c2 {
                    cnt += 1;
                }
            }
            cnt <= 2
        }
        for i in 0..strs.len() {
            let pi = find(&mut uf, i);
            for j in i + 1..strs.len() {
                let pj = find(&mut uf, j);
                if pi == pj {
                    continue;
                }
                if check(&strs[i], &strs[j]) {
                    uf[pj] = pi;
                    ans -= 1;
                }
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
    fn num_similar_groups() {
        assert_eq!(
            Solution::num_similar_groups(vec_str!["tars", "rats", "arts", "star"]),
            2
        );
        assert_eq!(Solution::num_similar_groups(vec_str!["omv", "ovm"]), 1);
    }
}
