// https://leetcode.com/problems/minimum-operations-to-make-array-elements-zero/
// 3495. Minimum Operations to Make Array Elements Zero
pub struct Solution;
impl Solution {
    pub fn min_operations(queries: Vec<Vec<i32>>) -> i64 {
        let mut ans = 0;
        let mut v = vec![0; 16];
        for i in 1..16 {
            v[i] = i as i64 * ((1 << (2 * i)) - (1 << (2 * i - 2))) + v[i - 1];
        }
        for q in queries {
            let l = q[0] as i64;
            let r = q[1] as i64;
            let nl = (i64::BITS - l.leading_zeros() + 1) as i64 / 2;
            let nr = (i64::BITS - r.leading_zeros() + 1) as i64 / 2;
            let nq = if nl == nr {
                nl * (r - l + 1)
            } else {
                nl * ((1 << (nl * 2)) - l) + nr * (r - (1 << (nr * 2 - 2)) + 1) + v[nr as usize - 1] - v[nl as usize]
            };
            ans += (nq + 1) / 2;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn min_operations() {
        assert_eq!(Solution::min_operations(vec_vec![[1, 21]]), 23);
        assert_eq!(Solution::min_operations(vec_vec![[1, 8]]), 7);
        assert_eq!(Solution::min_operations(vec_vec![[1, 2], [2, 4]]), 3);
        assert_eq!(Solution::min_operations(vec_vec![[2, 6]]), 4);
    }
}
