// https://leetcode.com/problems/count-the-number-of-ideal-arrays/
// 2338. Count the Number of Ideal Arrays
pub struct Solution;
const MOD: i64 = 1000000007;
impl Solution {
    pub fn ideal_arrays(n: i32, max_value: i32) -> i32 {
        let mut ans = max_value as i64;
        let mut cnt = vec![1; max_value as usize + 1];
        let mut nsplit = 1;
        let div = |mut a: i64, mut b: i64| -> i64 {
            while a % b != 0 {
                let n = MOD / b + 1;
                a = a * n % MOD;
                b = b * n % MOD;
            }
            a / b
        };
        for sz in 2..=n as usize {
            let mut ncnt = vec![0; max_value as usize + 1];
            let mut has_nz = false;
            let mut cnt_this = 0;
            for i in 1..=max_value as usize / 2 {
                if cnt[i] == 0 {
                    continue;
                }
                has_nz = true;
                for j in 2.. {
                    if i * j <= max_value as usize {
                        ncnt[i * j] = (ncnt[i * j] + cnt[i]) % MOD;
                        cnt_this = (cnt_this + cnt[i]) % MOD;
                    } else {
                        break;
                    }
                }
            }
            if !has_nz {
                break;
            }
            cnt = ncnt;
            nsplit = (nsplit * (n as i64 - sz as i64 + 1)) % MOD;
            nsplit = div(nsplit, sz as i64 - 1);
            ans = (ans + cnt_this * nsplit) % MOD;
        }
        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ideal_arrays() {
        assert_eq!(Solution::ideal_arrays(2, 5), 10);
        assert_eq!(Solution::ideal_arrays(5, 3), 11);
    }
}
