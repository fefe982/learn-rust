// https://leetcode.com/problems/number-of-zigzag-arrays-i/
// 3699. Number of Zigzag Arrays I
pub struct Solution;
const MOD: i64 = 1_000_000_007;
impl Solution {
    pub fn zig_zag_arrays(n: i32, l: i32, r: i32) -> i32 {
        let len = (r - l) as usize;
        let mut v = (1..=len as i64).collect::<Vec<i64>>();
        // let mut v1 = vec![0; len];
        for i in 1..n {
            if i % 2 == 1 {
                for j in (1..len).rev() {
                    v[j - 1] = (v[j - 1] + v[j]) % MOD;
                }
            } else {
                for j in 1..len {
                    v[j] = (v[j] + v[j - 1]) % MOD;
                }
            }
        }
        ((if n % 2 == 1 { v[len - 1] } else { v[0] }) * 2 % MOD) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn zig_zag_arrays() {
        assert_eq!(Solution::zig_zag_arrays(21, 8, 270), 495526556);
        assert_eq!(Solution::zig_zag_arrays(3, 4, 5), 2);
        assert_eq!(Solution::zig_zag_arrays(3, 1, 3), 10);
    }
}
