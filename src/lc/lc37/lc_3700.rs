// https://leetcode.com/problems/number-of-zigzag-arrays-ii/
// 3700. Number of Zigzag Arrays II
pub struct Solution;
const MOD: i64 = 1_000_000_007;
impl Solution {
    fn matmul(a: &Vec<Vec<i64>>, b: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
        let mut c = vec![vec![0; b[0].len()]; a.len()];
        for i in 0..a.len() {
            for j in 0..b[0].len() {
                for k in 0..b.len() {
                    c[i][j] = (c[i][j] + a[i][k] * b[k][j]) % MOD;
                }
            }
        }
        c
    }
    pub fn zig_zag_arrays(n: i32, l: i32, r: i32) -> i32 {
        let len = (r - l) as usize;
        let n = n - 1;
        let mut v = vec![vec![0; len]; len];
        for i in 0..len {
            for j in 0..len {
                if i < j {
                    v[i][j] = (len - j) as i64;
                } else {
                    v[i][j] = (len - i) as i64;
                }
            }
        }
        let mut factor = 2;
        let mut vres = vec![vec![0; len]; len];
        for i in 0..len {
            vres[i][i] = 1;
        }
        loop {
            if n & factor != 0 {
                vres = Self::matmul(&vres, &v);
            }
            factor <<= 1;
            if factor > n {
                break;
            }
            v = Self::matmul(&v, &v);
        }
        let mut c = vec![0; len];
        for i in 0..len {
            for j in 0..len {
                c[i] = (c[i] + vres[i][j]) % MOD;
            }
        }
        if n & 1 == 1 {
            for i in 1..len {
                c[i] = (c[i] + c[i - 1]) % MOD;
            }
        }
        let mut r = 0;
        for i in 0..len {
            r = (r + c[i]) % MOD;
        }
        (r * 2 % MOD) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn zig_zag_arrays() {
        assert_eq!(Solution::zig_zag_arrays(3, 4, 5), 2);
        assert_eq!(Solution::zig_zag_arrays(3, 1, 3), 10);
    }
}
