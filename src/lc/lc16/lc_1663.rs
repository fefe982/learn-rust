// https://leetcode.com/problems/smallest-string-with-a-given-numeric-value/
// 1663. Smallest String With A Given Numeric Value
pub struct Solution;
impl Solution {
    pub fn get_smallest_string(n: i32, k: i32) -> String {
        let a = 'a' as u8;
        let mut str = Vec::with_capacity(n as usize);
        let m = (k - n) / 25;
        let r = (k - n) % 25;
        for _ in 1..(n - m) {
            str.push(a)
        }
        if m < n {
            str.push(a + r as u8)
        }
        for _ in 0..m {
            str.push('z' as u8)
        }
        String::from_utf8(str).unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_smallest_string() {
        assert_eq!(Solution::get_smallest_string(3, 27), "aay");
        assert_eq!(Solution::get_smallest_string(5, 73), "aaszz");
    }
}
