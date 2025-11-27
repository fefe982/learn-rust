// https://leetcode.com/problems/apply-operations-to-maximize-score/
// 2818. Apply Operations to Maximize Score
pub struct Solution;
const MOD: i64 = 1_000_000_007;
impl Solution {
    fn get_primes() -> Vec<i32> {
        let mut v = vec![true; 316];
        let mut r = vec![];
        for i in 2..=17 {
            if v[i] {
                r.push(i as i32);
                for j in 2..=315 / i {
                    v[i * j] = false;
                }
            }
        }
        for i in 19..316 {
            if v[i] {
                r.push(i as i32);
            }
        }
        r
    }
    fn prime_count(mut n: i32, primes: &Vec<i32>) -> i32 {
        let mut r = 0;
        let mut i = 0;
        while n > 1 && i < primes.len() {
            if n % primes[i] == 0 {
                r += 1;
                while (n % primes[i]) == 0 {
                    n /= primes[i];
                }
            }
            i += 1;
        }
        if n > 1 {
            r + 1
        } else {
            r
        }
    }
    fn pow(a: i32, mut b: i64) -> i64 {
        let mut a = a as i64;
        let mut r = 1;
        while b > 0 {
            if b & 1 == 1 {
                r = (r * a) % MOD;
            }
            a = (a * a) % MOD;
            b >>= 1;
        }
        r
    }
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        let primes = Self::get_primes();
        let mut cnt = vec![0; nums.len()];
        let mut v = vec![(i32::MAX, 0)];
        for i in 0..nums.len() {
            let pi = Self::prime_count(nums[i], &primes);
            while let Some(&(lc, pos)) = v.last() {
                if lc < pi {
                    v.pop();
                    cnt[pos - 1] += (pos - v.last().unwrap().1) as i64 * (i + 1 - pos) as i64;
                } else {
                    break;
                }
            }
            v.push((pi, i + 1));
        }
        for i in 1..v.len() {
            cnt[v[i].1 - 1] += (v[i].1 - v[i - 1].1) as i64 * (nums.len() + 1 - v[i].1) as i64;
        }
        let mut nums = nums.into_iter().zip(cnt.into_iter()).collect::<Vec<_>>();
        nums.sort_by(|a, b| b.0.cmp(&a.0));
        let mut k = k as i64;
        let mut i = 0;
        let mut ans = 1;
        while k > 0 {
            if nums[i].1 >= k {
                ans = (ans * Self::pow(nums[i].0, k)) % MOD;
                break;
            } else {
                ans = (ans * Self::pow(nums[i].0, nums[i].1)) % MOD;
                k -= nums[i].1;
            }
            i += 1;
        }
        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_score() {
        assert_eq!(Solution::maximum_score(vec![8, 3, 9, 3, 8], 2), 81);
        assert_eq!(Solution::maximum_score(vec![19, 12, 14, 6, 10, 18], 3), 4788);
    }
}
