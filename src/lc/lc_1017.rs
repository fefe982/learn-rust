// https://leetcode.cn/problems/convert-to-base-2/
// 1017. Convert to Base -2
pub struct Solution;
impl Solution {
    pub fn base_neg2(mut n: i32) -> String {
        if n == 0 {
            return String::from("0");
        }
        let mut repr = Vec::new();
        let mut idx = 0;
        while n > 0 {
            repr.push(b'0' + (n % 2) as u8);
            if n % 2 != 0 && idx % 2 != 0 {
                n += 2;
            }
            n /= 2;
            idx += 1;
        }
        String::from_utf8(repr.into_iter().rev().collect()).unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn base_neg2() {
        assert_eq!(Solution::base_neg2(2), String::from("110"));
        assert_eq!(Solution::base_neg2(3), String::from("111"));
        assert_eq!(Solution::base_neg2(4), String::from("100"));
    }
}
