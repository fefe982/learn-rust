// https://leetcode.com/problems/partition-string-into-substrings-with-values-at-most-k/
// 2522. Partition String Into Substrings With Values at Most K
pub struct Solution;
impl Solution {
    pub fn minimum_partition(s: String, k: i32) -> i32 {
        let k = k as i64;
        let mut ans = 0;
        let mut cur = 0;
        for c in s.chars() {
            let c = c as i64 - '0' as i64;
            cur = cur * 10 + c;
            if cur > k {
                ans += 1;
                cur = c;
            }
            if cur > k {
                return -1;
            }
        }
        ans + 1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_partition() {
        assert_eq!(Solution::minimum_partition("165462".to_string(), 60), 4);
        assert_eq!(Solution::minimum_partition("238182".to_string(), 5), -1);
    }
}
