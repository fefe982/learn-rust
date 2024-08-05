// https://leetcode.com/problems/kth-distinct-string-in-an-array/
// 2053. Kth Distinct String in an Array
pub struct Solution;
impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let mut cnt = std::collections::HashMap::<String, i32>::new();
        for s in &arr {
            *cnt.entry(s.clone()).or_insert(0) += 1;
        }
        let mut c = 0;
        for s in arr {
            if cnt[&s] == 1 {
                c += 1;
                if c == k {
                    return s.clone();
                }
            }
        }
        "".to_string()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_kth_distinct() {
        assert_eq!(Solution::kth_distinct(vec_str!["d", "b", "c", "b", "c", "a"], 2), "a");
        assert_eq!(Solution::kth_distinct(vec_str!["aaa", "aa", "a"], 1), "aaa");
        assert_eq!(Solution::kth_distinct(vec_str!["a", "b", "a"], 3), "");
    }
}
