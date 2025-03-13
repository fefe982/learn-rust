// https://leetcode.com/problems/check-balanced-string/
// 3340. Check if There is a Valid Partition For The Array
pub struct Solution;
impl Solution {
    pub fn is_balanced(num: String) -> bool {
        let num = num.as_bytes();
        let mut sum = [0, 0];
        for (i, &n) in num.iter().enumerate() {
            sum[i % 2] += (n - b'0') as i32;
        }
        sum[0] == sum[1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_balanced() {
        assert_eq!(Solution::is_balanced("1234".to_string()), false);
        assert_eq!(Solution::is_balanced("24123".to_string()), true);
    }
}
