// https://leetcode.com/problems/count-good-strings/
// 4073. Count Good Strings
pub struct Solution;
impl Solution {
    pub fn count_good_strings(n: i64) -> i32 {
        const MOD: i64 = 1_000_000_007;
        fn mul(a: [[i64; 2]; 2], b: [[i64; 2]; 2]) -> [[i64; 2]; 2] {
            let mut c = [[0; 2]; 2];
            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        c[i][j] = (c[i][j] + a[i][k] * b[k][j]) % MOD;
                    }
                }
            }
            c
        }
        fn mulv(a: [[i64; 2]; 2], v: [i64; 2]) -> [i64; 2] {
            [
                (a[0][0] * v[0] % MOD + a[0][1] * v[1] % MOD) % MOD,
                (a[1][0] * v[0] % MOD + a[1][1] * v[1] % MOD) % MOD,
            ]
        }
        let mut a = [[1, 1], [1, 0]];
        let mut v = [2, 0];
        let mut n = n - 1;
        while n > 0 {
            if n & 1 == 1 {
                v = mulv(a, v);
            }
            a = mul(a, a);
            n >>= 1;
        }
        v[0] as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_good_strings() {
        assert_eq!(Solution::count_good_strings(4), 6);
        assert_eq!(Solution::count_good_strings(3), 4);
        assert_eq!(Solution::count_good_strings(2), 2);
    }
}
