// https://leetcode.com/problems/maximum-good-people-based-on-statements/
// 2151. Maximum Good People Based on Statements
pub struct Solution;
impl Solution {
    pub fn maximum_good(statements: Vec<Vec<i32>>) -> i32 {
        let n = statements.len();
        let mut good = vec![0; n];
        let mut bad = vec![0; n];
        for i in 0..n {
            for j in 0..n {
                match statements[i][j] {
                    0 => bad[i] |= 1 << j,
                    1 => good[i] |= 1 << j,
                    _ => (),
                }
            }
        }
        let mut ans = 0;
        'm: for mask in 1..(1 << n) as i32 {
            for i in 0..n {
                if mask & (1 << i) == 0 {
                    continue;
                }
                if (good[i] & mask) != good[i] || (bad[i] & mask != 0) {
                    continue 'm;
                }
            }
            ans = ans.max(mask.count_ones() as i32);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_maximum_good() {
        assert_eq!(Solution::maximum_good(vec_vec![[2, 1, 2], [1, 2, 2], [2, 0, 2]]), 2);
        assert_eq!(Solution::maximum_good(vec_vec![[2, 0], [0, 2]]), 1);
    }
}
