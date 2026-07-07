// https://leetcode.com/problems/concatenate-non-zero-digits-and-multiply-by-sum-ii/
// 3756. Concatenate Non-Zero Digits and Multiply by Sum II
pub struct Solution;
impl Solution {
    pub fn sum_and_multiply(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let s = s.as_bytes();
        let mut sum = vec![0; s.len() + 1];
        let mut val = vec![0; s.len() + 1];
        let mut cnt = vec![0; s.len() + 1];
        for i in 0..s.len() {
            let d = (s[i] - b'0') as i64;
            if d == 0 {
                cnt[i + 1] = cnt[i];
                sum[i + 1] = sum[i];
                val[i + 1] = val[i];
            } else {
                cnt[i + 1] = cnt[i] + 1;
                sum[i + 1] = sum[i] + d;
                val[i + 1] = (val[i] * 10 + d) % MOD;
            }
        }
        let mut pow10 = vec![0; cnt[s.len()] + 1];
        pow10[0] = 1;
        const MOD: i64 = 1_000_000_007;
        for i in 1..pow10.len() {
            pow10[i] = pow10[i - 1] * 10 % MOD;
        }
        let mut ans = Vec::with_capacity(queries.len());
        for q in queries {
            let l = q[0] as usize;
            let r = q[1] as usize;
            let c = cnt[r + 1] - cnt[l];
            let x = (val[r + 1] - val[l] * pow10[c] % MOD + MOD) % MOD;
            let s = (sum[r + 1] - sum[l] + MOD) % MOD;
            ans.push((x * s % MOD) as i32);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn sum_and_multiply() {
        assert_eq!(
            Solution::sum_and_multiply("10203004".to_string(), vec_vec![[0, 7], [1, 3], [4, 6]]),
            vec![12340, 4, 9]
        );
        assert_eq!(
            Solution::sum_and_multiply("1000".to_string(), vec_vec![[0, 3], [1, 1]]),
            vec![1, 0]
        );
    }
}
