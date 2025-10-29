// https://leetcode.com/problems/count-ways-to-choose-coprime-integers-from-rows/
// 3725. Count Ways to Choose Coprime Integers From Rows
pub struct Solution;
const MOD: i64 = 1_000_000_007;
impl Solution {
    pub fn count_coprime(mat: Vec<Vec<i32>>) -> i32 {
        let mx = mat.iter().map(|r| *r.iter().max().unwrap()).max().unwrap() as usize;
        let mut dp = vec![1i64; mx + 1];
        let mut cnt = vec![0i64; mx + 1];
        for row in mat {
            cnt.fill(0);
            for x in row {
                cnt[x as usize] += 1;
            }
            for x in 1..=mx {
                for j in (x + x..=mx).step_by(x) {
                    cnt[x] += cnt[j];
                }
                dp[x] = dp[x] * cnt[x] % MOD;
            }
        }
        for x in (1..=mx).rev() {
            for j in (x + x..=mx).step_by(x) {
                dp[x] = dp[x] - dp[j]
            }
            dp[x] = (dp[x] % MOD + MOD) % MOD;
        }
        dp[1] as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn count_coprime() {
        assert_eq!(Solution::count_coprime(vec_vec![[1, 2], [3, 4]]), 3);
        assert_eq!(Solution::count_coprime(vec_vec![[2, 2], [2, 2]]), 0);
    }
}
