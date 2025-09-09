// https://leetcode.com/problems/remove-all-adjacent-duplicates-in-string-ii/
// 1209. Remove All Adjacent Duplicates in String II
pub struct Solution;
impl Solution {
    pub fn remove_duplicates(s: String, k: i32) -> String {
        let mut stack = vec![];
        for c in s.chars() {
            if let Some(&(cnt, ch)) = stack.last() {
                if ch == c {
                    if cnt + 1 == k {
                        for _ in 0..k - 1 {
                            stack.pop();
                        }
                    } else {
                        stack.push((cnt + 1, c));
                    }
                } else {
                    stack.push((1, c));
                }
            } else {
                stack.push((1, c));
            }
        }
        stack.into_iter().map(|(_, c)| c).collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn remove_duplicates() {
        assert_eq!(Solution::remove_duplicates("abcd".to_string(), 2), "abcd".to_string());
        assert_eq!(
            Solution::remove_duplicates("deeedbbcccbdaa".to_string(), 3),
            "aa".to_string()
        );
        assert_eq!(Solution::remove_duplicates("pbbcggttciiippooaais".to_string(), 2), "ps");
    }
}
