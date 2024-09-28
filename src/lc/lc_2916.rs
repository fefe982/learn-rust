// https://leetcode.com/problems/subarrays-distinct-element-sum-of-squares-ii/
// 2916. Subarrays Distinct Element Sum of Squares II
pub struct Solution;
const MOD: i64 = 1_0000_0000_7;
impl Solution {
    fn get(v: &mut Vec<(i64, i64)>, idx: usize, left: i64, right: i64, ql: i64, qr: i64, inc: i64) -> i64 {
        v[idx].0 += inc;
        v[idx].1 += inc * (right - left);
        if ql <= left && qr >= right {
            let res = v[idx].1;
            v[idx].0 += 1;
            v[idx].1 += right - left;
            return res % MOD;
        }
        if qr <= left || ql >= right {
            return 0;
        }
        let mid = (left + right) >> 1;
        let inc = v[idx].0;
        v[idx].0 = 0;
        let res =
            Self::get(v, idx * 2 + 1, left, mid, ql, qr, inc) + Self::get(v, idx * 2 + 2, mid, right, ql, qr, inc);
        v[idx].1 = v[idx * 2 + 1].1 + v[idx * 2 + 2].1;
        res % MOD
    }
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut len = 1;
        while len < n {
            len <<= 1;
        }
        let mut v = vec![(0, 0); len * 2];
        let mut last_idx_map = std::collections::HashMap::<i32, usize>::new();
        let mut partial = 0;
        let mut sum = 0;
        for (i, n) in nums.into_iter().enumerate() {
            let last_idx = last_idx_map.get(&n).cloned().unwrap_or(0) as i64;
            last_idx_map.insert(n, i + 1);
            partial +=
                (2 * Self::get(&mut v, 0, 0, len as i64, last_idx, i as i64 + 1, 0) + i as i64 - last_idx + 1) % MOD;
            sum = (sum + partial) % MOD;
        }
        sum as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sum_counts() {
        assert_eq!(Solution::sum_counts(vec![1, 2, 1]), 15);
        assert_eq!(Solution::sum_counts(vec![2, 2]), 3);
    }
}
