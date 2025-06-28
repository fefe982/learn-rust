// https://leetcode.com/problems/the-k-th-lexicographical-string-of-all-happy-strings-of-length-n/
// 1415. The k-th Lexicographical String of All Happy Strings of Length n
pub struct Solution;
impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        let mut s = 1 << (n - 1);
        if k > 3 * s {
            return "".to_string();
        }
        let mut res = "".to_string();
        let mut k = k - 1;
        let mut c = k / s;
        k = k % s;
        res.push((b'a' + c as u8) as char);
        for _ in 1..n {
            s >>= 1;
            let nc = k / s;
            k = k % s;
            c = match (c, nc) {
                (0, 0) => 1,
                (0, 1) => 2,
                (1, 0) => 0,
                (1, 1) => 2,
                (2, 0) => 0,
                (2, 1) => 1,
                _ => unreachable!(),
            };
            res.push((b'a' + c as u8) as char);
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_happy_string() {
        assert_eq!(Solution::get_happy_string(1, 3), String::from("c"));
        assert_eq!(Solution::get_happy_string(3, 9), String::from("cab"));
    }
}
