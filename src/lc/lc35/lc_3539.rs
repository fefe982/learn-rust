// https://leetcode.com/problems/find-sum-of-array-product-of-magical-sequences/
// 3539. Find Sum of Array Product of Magical Sequences
pub struct Solution;
const MOD: i64 = 1_000_000_007;
impl Solution {
    fn div(mut x: i64, mut y: i64) -> i64 {
        while x % y != 0 {
            let t = MOD / y + 1;
            x = (x * t) % MOD;
            y = (y * t) % MOD;
        }
        x / y
    }
    pub fn magical_sum(m: i32, k: i32, nums: Vec<i32>) -> i32 {
        let m = m as usize;
        let k = k as usize;
        let mut frac = vec![1i64; m + 1];
        let mut ifrac = vec![1i64; m + 1];
        for i in 2..=m {
            frac[i] = frac[i - 1] * i as i64 % MOD;
            ifrac[i] = Self::div(1, frac[i]);
        }
        let mut nump = vec![vec![1; nums.len()]; m + 1];
        for i in 0..nums.len() {
            for j in 1..=m {
                nump[j][i] = (nump[j - 1][i] * nums[i] as i64) % MOD;
            }
        }
        let mut v = vec![vec![vec![vec![0; k + 1]; m + 1]; m + 1]; nums.len()];
        for i in 0..=m {
            v[0][i][i][0] = nump[i][0] * ifrac[i] % MOD;
        }
        for i in 1..nums.len() {
            for j in 0..=m {
                for p in 0..=m {
                    for q in 0..=k {
                        let nq = q + p % 2;
                        if nq > k {
                            break;
                        }
                        if v[i - 1][j][p][q] == 0 {
                            continue;
                        }
                        for r in 0..=m - j {
                            let np = p / 2 + r;
                            v[i][j + r][np][nq] =
                                (v[i][j + r][np][nq] + v[i - 1][j][p][q] * nump[r][i] % MOD * ifrac[r] % MOD) % MOD;
                        }
                    }
                }
            }
        }
        let mut ans = 0;
        for p in 0..=m {
            for q in 0..=k {
                if (p.count_ones() as usize + q) == k {
                    ans = (ans + v[nums.len() - 1][m][p][q]) % MOD;
                }
            }
        }
        (ans * frac[m] % MOD) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn magical_sum() {
        assert_eq!(Solution::magical_sum(5, 5, vec![1, 10, 100, 10000, 1000000]), 991600007);
        assert_eq!(Solution::magical_sum(2, 2, vec![5, 4, 3, 2, 1]), 170);
        assert_eq!(Solution::magical_sum(1, 1, vec![28]), 28);
    }
}
