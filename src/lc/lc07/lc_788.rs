// https://leetcode.com/problems/rotated-digits/description/
// 788. Rotated Digits
pub struct Solution;
impl Solution {
    pub fn rotated_digits(n: i32) -> i32 {
        let n3 = [0, 1, 1, 1, 1, 1, 1, 1, 2, 2];
        let n7 = [0, 1, 2, 2, 2, 3, 4, 4, 5, 6];
        let mut n = n;
        let mut c7 = 0;
        let mut c3 = 0;
        let mut m3 = 1;
        let mut m7 = 1;
        while n > 0 {
            let d = (n % 10) as usize;
            n /= 10;
            if d > 0 {
                if n3[d - 1] == n3[d] {
                    c3 = m3 - 1;
                }
                if n7[d - 1] == n7[d] {
                    c7 = m7 - 1;
                }
            }
            c7 += n7[d] * m7;
            c3 += n3[d] * m3;
            m3 *= 3;
            m7 *= 7;
        }
        c7 - c3
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rotated_digits() {
        assert_eq!(Solution::rotated_digits(857), 247);
        assert_eq!(Solution::rotated_digits(10), 4);
        assert_eq!(Solution::rotated_digits(1), 0);
        assert_eq!(Solution::rotated_digits(2), 1);
    }
}
