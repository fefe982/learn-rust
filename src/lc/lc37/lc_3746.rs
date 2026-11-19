// https://leetcode.com/problems/minimum-string-length-after-balanced-removals/
// 3746. Minimum String Length After Balanced Removals
pub struct Solution;
impl Solution {
    pub fn min_length_after_removals(s: String) -> i32 {
        let mut d: i32 = 0;
        for c in s.chars() {
            if c == 'a' {
                d += 1;
            } else {
                d -= 1;
            }
        }
        d.abs()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_length_after_removals() {
        assert_eq!(Solution::min_length_after_removals("aabbab".to_string()), 0);
        assert_eq!(Solution::min_length_after_removals("aaaa".to_string()), 4);
        assert_eq!(Solution::min_length_after_removals("aaabb".to_string()), 1);
    }
}
