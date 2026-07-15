// https://leetcode.com/problems/construct-the-longest-new-string/
// 2745. Construct the Longest New String
pub struct Solution;
impl Solution {
    pub fn longest_string(x: i32, y: i32, z: i32) -> i32 {
        let mut n = x.min(y) * 2;
        if x != y {
            n += 1;
        }
        (n + z) * 2
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn longest_string() {
        assert_eq!(Solution::longest_string(2, 5, 1), 12);
        assert_eq!(Solution::longest_string(3, 2, 2), 14);
    }
}
