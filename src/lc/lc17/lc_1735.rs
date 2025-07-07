// https://leetcode.com/problems/count-ways-to-make-array-with-product/
// 1735. Count Ways to Make Array With Product
pub struct Solution;
const MOD: i64 = 1_0000_0000_7;
impl Solution {
    fn frac(n: usize, v: &mut Vec<i64>) -> i64 {
        while n >= v.len() {
            let l = v.len();
            v.push(v[l - 1] * l as i64 % MOD);
        }
        return v[n];
    }
    fn div(mut a: i64, mut b: i64) -> i64 {
        while b > 1 {
            let t = MOD / b + 1;
            a = a * t % MOD;
            b = b * t % MOD;
        }
        a
    }
    fn cnt(sz: usize, mut n: usize, v: &mut Vec<i64>) -> i32 {
        let mut ans = 1;
        let mut d = 2;
        while d * d <= n {
            if n % d == 0 {
                let mut cnt = 0;
                while n % d == 0 {
                    cnt += 1;
                    n /= d;
                }
                ans =
                    ans * Self::div(
                        Self::div(Self::frac(sz + cnt - 1, v), Self::frac(cnt, v)),
                        Self::frac(sz - 1, v),
                    ) % MOD;
            }
            d += 1;
        }
        if n > 1 {
            ans = ans * (sz as i64) % MOD;
        }
        ans as i32
    }
    pub fn ways_to_fill_array(queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut v = vec![1];
        queries
            .into_iter()
            .map(|q| Self::cnt(q[0] as usize, q[1] as usize, &mut v))
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_ways_to_fill_array() {
        assert_eq!(
            Solution::ways_to_fill_array(vec_vec![[2, 6], [5, 1], [73, 660]]),
            [4, 1, 50734910]
        );
        assert_eq!(
            Solution::ways_to_fill_array(vec_vec![[1, 1], [2, 2], [3, 3], [4, 4], [5, 5]]),
            [1, 2, 3, 10, 5]
        );
    }
}
