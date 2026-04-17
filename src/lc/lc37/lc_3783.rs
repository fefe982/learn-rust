// https://leetcode.com/problems/mirror-distance-of-an-integer/
// 3783. Mirror Distance of an Integer
pub struct Solution;
impl Solution {
    pub fn mirror_distance(n: i32) -> i32 {
        let mut rev = 0_i64;
        let mut x = n as i64;
        while x > 0 {
            rev = rev * 10 + x % 10;
            x /= 10;
        }
        ((n as i64) - rev).abs() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mirror_distance() {
        assert_eq!(Solution::mirror_distance(25), 27);
        assert_eq!(Solution::mirror_distance(10), 9);
        assert_eq!(Solution::mirror_distance(7), 0);
    }
}
