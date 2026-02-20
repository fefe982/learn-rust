// https://leetcode.com/problems/check-if-digits-are-equal-in-string-after-operations-ii/
// 3463. Check if Digits of a Number are in Increasing Order
pub struct Solution;
impl Solution {
    pub fn has_same_digits(s: String) -> bool {
        let mut n2 = 0;
        let mut n5 = 0;
        let d5 = [0, 1, 3, 2, 4];
        let s = s.as_bytes();
        let l = s.len() as i32 - 2;
        let mut v5 = 1;
        let mut val02 = (s[0] - b'0') as i32 % 2;
        let mut val05 = (s[0] - b'0') as i32 % 5;
        let mut val12 = (s[1] - b'0') as i32 % 2;
        let mut val15 = (s[1] - b'0') as i32 % 5;
        for i in 1..s.len() - 1 {
            let d = l - i as i32 + 1;
            let mut dd2 = d;
            while dd2 % 2 == 0 {
                dd2 /= 2;
                n2 += 1;
            }
            let mut dd5 = d;
            while dd5 % 5 == 0 {
                dd5 /= 5;
                n5 += 1;
            }
            let mut ddi2 = i as i32;
            while ddi2 % 2 == 0 {
                ddi2 /= 2;
                n2 -= 1;
            }
            let mut ddi5 = i as i32;
            while ddi5 % 5 == 0 {
                ddi5 /= 5;
                n5 -= 1;
            }
            v5 = (v5 * (dd5 % 5) * d5[ddi5 as usize % 5]) % 5;
            if n2 == 0 {
                val02 = (val02 + (s[i] - b'0') as i32 % 2) % 2;
                val12 = (val12 + (s[i + 1] - b'0') as i32 % 2) % 2;
            }
            if n5 == 0 {
                val05 = (val05 + (s[i] - b'0') as i32 % 5 * v5) % 5;
                val15 = (val15 + (s[i + 1] - b'0') as i32 % 5 * v5) % 5;
            }
        }
        val02 == val12 && val05 == val15
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn has_same_digits() {
        assert_eq!(Solution::has_same_digits("3902".to_string()), true);
        assert_eq!(Solution::has_same_digits("34789".to_string()), false);
    }
}
