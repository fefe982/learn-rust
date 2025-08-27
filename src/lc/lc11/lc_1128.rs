// https://leetcode.com/problems/number-of-equivalent-domino-pairs
// 1128. Number of Equivalent Domino Pairs
pub struct Solution;
impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut m: std::collections::HashMap<(i32, i32), i32> = std::collections::HashMap::new();
        let mut ans = 0;
        for d in dominoes {
            let k = if d[0] < d[1] { (d[0], d[1]) } else { (d[1], d[0]) };
            let v = m.entry(k).or_insert(0);
            ans += *v;
            *v += 1;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn num_equiv_domino_pairs() {
        assert_eq!(
            Solution::num_equiv_domino_pairs(vec_vec![[1, 2], [2, 1], [3, 4], [5, 6]]),
            1
        );
        assert_eq!(
            Solution::num_equiv_domino_pairs(vec_vec![[1, 2], [1, 2], [1, 1], [1, 2], [2, 2]]),
            3
        );
    }
}
