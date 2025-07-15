// https://leetcode.com/problems/count-substrings-that-satisfy-k-constraint-ii/
// 3261. Count Substrings That Satisfy K Constraint II
pub struct Solution;
impl Solution {
    pub fn count_k_constraint_substrings(s: String, k: i32, queries: Vec<Vec<i32>>) -> Vec<i64> {
        let s = s.as_bytes();
        let mut ridx = vec![];
        let mut cnt0 = 0;
        let mut cnt1 = 0;
        let mut i = 0;
        let mut j = 0;
        while j < s.len() {
            while j < s.len() && (cnt0 <= k || cnt1 <= k) {
                if s[j] == b'0' {
                    cnt0 += 1;
                } else {
                    cnt1 += 1;
                }
                j += 1;
            }
            while cnt0 > k && cnt1 > k {
                ridx.push(j - 1);
                if s[i] == b'0' {
                    cnt0 -= 1;
                } else {
                    cnt1 -= 1;
                }
                i += 1;
            }
        }
        let mut ans = Vec::with_capacity(queries.len());
        let mut sumj = Vec::with_capacity(ridx.len() + 1);
        sumj.push(0);
        for i in 0..ridx.len() {
            sumj.push(sumj[i] + ridx[i] as i64);
        }
        for q in queries {
            let l = q[0] as i64;
            let r = q[1] as i64;
            let mut cnt = (r - l + 1) * (r - l + 2) / 2;
            let ir = ridx.partition_point(|&x| x <= r as usize);
            if ir > l as usize {
                cnt -= (ir as i64 - l) * (r + 1) - (sumj[ir] - sumj[l as usize]);
            }
            ans.push(cnt);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn count_k_constraint_substrings() {
        assert_eq!(
            Solution::count_k_constraint_substrings(
                "000".to_string(),
                1,
                vec_vec![[0, 0], [0, 1], [0, 2], [1, 1], [1, 2], [2, 2]]
            ),
            [1, 3, 6, 1, 3, 1]
        );
        assert_eq!(
            Solution::count_k_constraint_substrings("0001111".to_string(), 2, vec_vec![[0, 6]]),
            [26]
        );
        assert_eq!(
            Solution::count_k_constraint_substrings("010101".to_string(), 1, vec_vec![[0, 5], [1, 4], [2, 3]]),
            [15, 9, 3]
        );
    }
}
