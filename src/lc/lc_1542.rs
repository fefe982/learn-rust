// https://leetcode.com/problems/find-longest-awesome-substring/
// 1542. Find Longest Awesome Substring
pub struct Solution;
impl Solution {
    pub fn longest_awesome(s: String) -> i32 {
        let mut map = std::collections::HashMap::new();
        map.insert(0, 0);
        let mut ans = 1;
        let mut num = 0;
        for (&c, i) in s.as_bytes().iter().zip(1..) {
            num = num ^ (1 << (c - b'0'));
            if let Some(&j) = map.get(&num) {
                ans = ans.max(i - j);
            } else {
                map.insert(num, i);
            }
            for d in 0..10 {
                let nn = num ^ (1 << d);
                if let Some(&j) = map.get(&nn) {
                    ans = ans.max(i - j);
                }
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_longest_awesome() {
        assert_eq!(Solution::longest_awesome("3242415".to_string()), 5);
        assert_eq!(Solution::longest_awesome("12345678".to_string()), 1);
        assert_eq!(Solution::longest_awesome("213123".to_string()), 6);
    }
}
