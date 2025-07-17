// https://leetcode.com/problems/minimum-time-to-remove-all-cars-containing-illegal-goods/
// 2167. Minimum Time to Remove All Cars Containing Illegal Goods
pub struct Solution;
impl Solution {
    pub fn minimum_time(s: String) -> i32 {
        let mut left = 0;
        let s = s.as_bytes();
        let n = s.len();
        let mut ans = n as i32;
        for i in 0..n {
            left = (left + if s[i] == b'1' { 1 } else { 0 } * 2).min(i as i32 + 1);
            ans = ans.min(left + (n - i) as i32 - 1);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_time() {
        assert_eq!(Solution::minimum_time("1100101".to_string()), 5);
        assert_eq!(Solution::minimum_time("0010".to_string()), 2);
    }
}
