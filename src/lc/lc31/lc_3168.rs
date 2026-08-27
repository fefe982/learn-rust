// https://leetcode.com/problems/minimum-number-of-chairs-in-a-waiting-room/
// 3168. Minimum Number of Chairs in a Waiting Room
pub struct Solution;
impl Solution {
    pub fn minimum_chairs(s: String) -> i32 {
        let mut max = 0;
        let mut cnt = 0;
        for c in s.chars() {
            if c == 'E' {
                cnt += 1;
                max = max.max(cnt);
            } else {
                cnt -= 1;
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_chairs() {
        assert_eq!(Solution::minimum_chairs("EEEEEEE".to_string()), 7);
        assert_eq!(Solution::minimum_chairs("ELELEEL".to_string()), 2);
        assert_eq!(Solution::minimum_chairs("ELEELEELLL".to_string()), 3);
    }
}
