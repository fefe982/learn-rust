// https://leetcode.com/problems/maximum-sum-of-subsequence-with-non-adjacent-elements/
// 3165. Maximum Sum of Subsequence With Non-Adjacent Elements
pub struct Solution;
impl Solution {
    fn insert(v: &mut Vec<(i64, i64, i64, i64)>, idx: usize, l: usize, r: usize, pos: usize, val: i32) {
        if l + 1 == r {
            // both vacant, left vacant, right vacant, no restriction
            v[idx] = (0, 0, 0, val.max(0) as i64);
            return;
        }
        let m = (l + r) / 2;
        if pos < m {
            Self::insert(v, idx * 2 + 1, l, m, pos, val);
        } else {
            Self::insert(v, idx * 2 + 2, m, r, pos, val);
        }
        v[idx] = (
            (v[idx * 2 + 1].0 + v[idx * 2 + 2].2).max(v[idx * 2 + 1].1 + v[idx * 2 + 2].0),
            (v[idx * 2 + 1].0 + v[idx * 2 + 2].3).max(v[idx * 2 + 1].1 + v[idx * 2 + 2].1),
            (v[idx * 2 + 1].2 + v[idx * 2 + 2].2).max(v[idx * 2 + 1].3 + v[idx * 2 + 2].0),
            (v[idx * 2 + 1].2 + v[idx * 2 + 2].3).max(v[idx * 2 + 1].3 + v[idx * 2 + 2].1),
        )
    }
    pub fn maximum_sum_subsequence(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let len = nums.len();
        let mut n = 1;
        while n < len {
            n *= 2;
        }
        let mut v = vec![(0, 0, 0, 0); 2 * n];
        for (i, x) in nums.into_iter().enumerate() {
            Self::insert(&mut v, 0, 0, n, i, x);
        }
        let mut res = 0;
        let md = 1_000_000_007;
        for q in queries.into_iter() {
            Self::insert(&mut v, 0, 0, n, q[0] as usize, q[1]);
            res = (res + v[0].3) % md;
        }
        res as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn maximum_sum_subsequence() {
        assert_eq!(
            Solution::maximum_sum_subsequence(vec![3, 5, 9], vec_vec![[1, -2], [0, -3]]),
            21
        );
        assert_eq!(Solution::maximum_sum_subsequence(vec![0, -1], vec_vec![[1, -5]]), 0);
    }
}
