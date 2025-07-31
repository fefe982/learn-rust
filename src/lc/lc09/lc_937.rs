// https://leetcode.com/problems/reorder-data-in-log-files/
// 937. Reorder Data in Log Files
pub struct Solution;
impl Solution {
    pub fn reorder_log_files(logs: Vec<String>) -> Vec<String> {
        let mut res = logs;
        res.sort_by_cached_key(|log| {
            let (l, r) = log.split_once(' ').unwrap();
            if r.chars().next().unwrap().is_alphabetic() {
                (r.to_string(), l.to_string())
            } else {
                ("~".to_string(), "~".to_string())
            }
        });
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn reorder_log_files() {
        assert_eq!(
            Solution::reorder_log_files(vec_str![
                "dig1 8 1 5 1",
                "let1 art can",
                "dig2 3 6",
                "let2 own kit dig",
                "let3 art zero"
            ]),
            vec_str![
                "let1 art can",
                "let3 art zero",
                "let2 own kit dig",
                "dig1 8 1 5 1",
                "dig2 3 6"
            ]
        );
        assert_eq!(
            Solution::reorder_log_files(vec_str![
                "a1 9 2 3 1",
                "g1 act car",
                "zo4 4 7",
                "ab1 off key dog",
                "a8 act zoo"
            ]),
            vec_str!["g1 act car", "a8 act zoo", "ab1 off key dog", "a1 9 2 3 1", "zo4 4 7"]
        )
    }
}
