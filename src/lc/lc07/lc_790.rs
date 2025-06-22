// https://leetcode.cn/problems/domino-and-tromino-tiling
// 790. Domino and Tromino Tiling
pub struct Solution;
impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        let mut v = [0; 1003];
        v[2] = 1;
        let n = n as usize;
        let mut s = 0;
        for i in 0..n {
            v[i + 3] = (2 * s + v[i + 1] + v[i + 2]) % 1_000_000_007i64;
            s = (s + v[i + 1]) % 1_000_000_007i64;
        }
        v[n + 2] as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn num_tilings() {
        assert_eq!(Solution::num_tilings(1), 1);
        assert_eq!(Solution::num_tilings(3), 5);
        assert_eq!(Solution::num_tilings(4), 11);
    }
}
