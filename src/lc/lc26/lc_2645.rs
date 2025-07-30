// https://leetcode.com/problems/minimum-additions-to-make-valid-string/
// 2645. Minimum Additions to Make Valid String
pub struct Solution;
impl Solution {
    pub fn add_minimum(word: String) -> i32 {
        let mut cnt = 0;
        let mut expect = 0;
        for &c in word.as_bytes() {
            let c = c - b'a';
            cnt += (c + 3 - expect) as i32 % 3;
            expect = (c + 1) % 3;
        }
        cnt + (3 - expect) as i32 % 3
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_minimum() {
        assert_eq!(Solution::add_minimum(String::from("b")), 2);
        assert_eq!(Solution::add_minimum(String::from("aaa")), 6);
        assert_eq!(Solution::add_minimum(String::from("abc")), 0);
    }
}
