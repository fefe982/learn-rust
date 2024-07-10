// https://leetcode.com/problems/crawler-log-folder/
// 1598. Crawler Log Folder
pub struct Solution;
impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let mut ans = 0;
        for log in logs {
            if log == "../" {
                ans = (ans - 1).max(0);
            } else if log != "./" {
                ans += 1;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_min_operations() {
        assert_eq!(Solution::min_operations(vec_str!["d1/", "d2/", "../", "d21/", "./"]), 2);
        assert_eq!(
            Solution::min_operations(vec_str!["d1/", "d2/", "./", "d3/", "../", "d31/"]),
            3
        );
        assert_eq!(Solution::min_operations(vec_str!["d1/", "../", "../", "../"]), 0);
    }
}
