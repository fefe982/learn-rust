// https://leetcode.com/problems/unique-paths/
// 62. Unique Paths
pub struct Solution;
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        if m == 1 || n == 1 {
            return 1;
        }
        let mut a = (m + n - 2) as i128;
        let mut b = (m.min(n) - 1) as i128;
        let mut n = 1i128;
        let mut d = 1i128;
        while b > 0 {
            n *= a;
            d *= b;
            a -= 1;
            b -= 1;
        }
        (n / d) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unique_paths() {
        assert_eq!(Solution::unique_paths(16, 16), 155117520);
        assert_eq!(Solution::unique_paths(51, 9), 1916797311);
        assert_eq!(Solution::unique_paths(3, 7), 28);
        assert_eq!(Solution::unique_paths(3, 2), 3);
    }
}
