// https://leetcode.com/problems/count-the-number-of-inversions/
// 3193. Count the Number of Inversions
pub struct Solution;
const MOD: i64 = 1_0000_0000_7;
impl Solution {
    fn div(mut a: i64, mut b: i64) -> i64 {
        while a % b != 0 {
            let n = MOD / b + 1;
            b = b * n % MOD;
            a = a * n % MOD;
        }
        a / b
    }
    pub fn number_of_permutations(n: i32, requirements: Vec<Vec<i32>>) -> i32 {
        let mut requirements = requirements;
        requirements.sort();
        let kmax = requirements.last().cloned().unwrap()[1] as usize + 1;
        let mut sum_cnt = vec![1; kmax];
        let mut ilast = 0;
        for r in requirements {
            for i in ilast..r[0] as usize {
                let mut nsum = sum_cnt.clone();
                for j in 1..(i + 1).min(kmax) {
                    nsum[j] = (sum_cnt[j] + nsum[j - 1]) % MOD;
                }
                for j in i + 1..kmax {
                    nsum[j] = (sum_cnt[j] - sum_cnt[j - i - 1] + MOD + nsum[j - 1]) % MOD;
                }
                sum_cnt = nsum;
            }
            let mut nsum = vec![0; kmax];
            let val = if r[1] < r[0] + 1 {
                sum_cnt[r[1] as usize]
            } else {
                (sum_cnt[r[1] as usize] - sum_cnt[r[1] as usize - r[0] as usize - 1] + MOD) % MOD
            };
            for i in r[1] as usize..kmax {
                nsum[i] = val;
            }
            sum_cnt = nsum;
            ilast = r[0] as usize + 1;
        }
        let mut res = sum_cnt[kmax - 1];
        for i in 0..n - ilast as i32 {
            res = (res * (n - i) as i64) % MOD;
            res = Self::div(res, (i + 1) as i64);
        }
        res as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_number_of_permutations() {
        assert_eq!(Solution::number_of_permutations(3, vec_vec![[2, 0]]), 1);
        assert_eq!(
            Solution::number_of_permutations(20, vec_vec![[5, 10], [6, 14], [19, 90], [18, 90], [3, 3], [2, 1]]),
            658544754
        );
        assert_eq!(Solution::number_of_permutations(3, vec_vec![[2, 2], [0, 0]]), 2);
        assert_eq!(Solution::number_of_permutations(3, vec_vec![[2, 2], [1, 1], [0, 0]]), 1);
    }
}
