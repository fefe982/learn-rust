// https://leetcode.com/problems/optimal-partition-of-string/
// 2405. Optimal Partition of String
pub struct Solution;
use std::collections::HashSet;
impl Solution {
    pub fn partition_string(s: String) -> i32 {
        let mut cnt = 0;
        let mut m = HashSet::new();
        for &c in s.as_bytes() {
            if m.contains(&c) {
                m.clear();
                cnt += 1;
            }
            m.insert(c);
        }
        cnt + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn partition_string() {
        assert_eq!(Solution::partition_string(String::from("abacaba")), 4);
        assert_eq!(Solution::partition_string(String::from("ssssss")), 6);
    }
}
