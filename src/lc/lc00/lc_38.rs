// https://leetcode.com/problems/count-and-say/
// 38. Count and Say
pub struct Solution;
impl Solution {
    fn cs(s: String) -> String {
        let mut last = '.';
        let mut r = "".to_string();
        let mut count = 0;
        for c in s.chars().chain([' '].into_iter()) {
            if c == last {
                count += 1;
            } else {
                if count > 0 {
                    r.push_str(format!("{}{}", count, last).as_str());
                }
                last = c;
                count = 1;
            }
        }
        r
    }
    pub fn count_and_say(n: i32) -> String {
        let mut r = "1".to_string();
        for _ in 1..n {
            r = Solution::cs(r);
        }
        r
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_and_say() {
        assert_eq!(Solution::count_and_say(1), "1".to_string());
        assert_eq!(Solution::count_and_say(4), "1211".to_string());
    }
}
