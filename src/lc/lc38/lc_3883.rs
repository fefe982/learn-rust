// https://leetcode.cn/problems/count-non-decreasing-arrays-with-given-digit-sums/
// 3883. Count Non-decreasing Arrays With Given Digit Sums
pub struct Solution;
impl Solution {
    pub fn count_arrays(digit_sum: Vec<i32>) -> i32 {
        const MAX_V: usize = 5000;
        const MAX_S: usize = 50;
        const MOD: i64 = 1_000_000_007;

        fn digit_sum_of(mut x: usize) -> usize {
            let mut s = 0usize;
            while x > 0 {
                s += x % 10;
                x /= 10;
            }
            s
        }

        let mut by_sum: Vec<Vec<usize>> = vec![Vec::new(); MAX_S + 1];
        for v in 0..=MAX_V {
            let s = digit_sum_of(v);
            by_sum[s].push(v);
        }

        let n = digit_sum.len();
        if n == 0 {
            return 0;
        }

        let mut prefix_sum = vec![0i64; MAX_V + 2];
        fn insert(prefix_sum: &mut Vec<i64>, mut idx: usize, val: i64) {
            idx = idx + 1;
            while idx < prefix_sum.len() {
                prefix_sum[idx] = (prefix_sum[idx] + val) % MOD;
                idx += idx & (!idx + 1);
            }
        }

        fn query(prefix_sum: &Vec<i64>, mut idx: usize) -> i64 {
            let mut ans = 0i64;
            idx = idx + 1;
            while idx > 0 {
                ans = (ans + prefix_sum[idx]) % MOD;
                idx -= idx & (!idx + 1);
            }
            ans
        }

        insert(&mut prefix_sum, 0, 1);

        for &s_i32 in digit_sum.iter() {
            let s = s_i32 as usize;
            if by_sum[s].is_empty() {
                return 0;
            }

            let mut prefix = vec![0i64; MAX_V + 2];

            for &v in &by_sum[s] {
                let val = query(&prefix_sum, v);
                insert(&mut prefix, v, val);
            }
            prefix_sum = prefix;
        }

        query(&prefix_sum, MAX_V) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_arrays() {
        assert_eq!(Solution::count_arrays(vec![4, 4]), 630);
        assert_eq!(Solution::count_arrays(vec![25, 1]), 6);
        assert_eq!(Solution::count_arrays(vec![1]), 4);
        assert_eq!(Solution::count_arrays(vec![2, 49, 23]), 0);
    }
}
