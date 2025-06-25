// https://leetcode.com/problems/naming-a-company/
// 2306. Naming a Company
pub struct Solution;
impl Solution {
    pub fn distinct_names(ideas: Vec<String>) -> i64 {
        let mut ch = vec![std::collections::HashSet::<Vec<char>>::new(); 26];
        ideas.iter().for_each(|s| {
            ch[s.chars().next().unwrap() as usize - 97].insert(s.chars().skip(1).collect::<Vec<_>>());
        });
        let mut cnt = 0;
        for i in 0..26 {
            for j in i + 1..26 {
                let intersect = ch[i].intersection(&ch[j]).count() as i64;
                cnt += (ch[i].len() as i64 - intersect) * (ch[j].len() as i64 - intersect) * 2;
            }
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_distinct_names() {
        assert_eq!(
            Solution::distinct_names(vec_str!["coffee", "donuts", "time", "toffee"]),
            6
        );
        assert_eq!(Solution::distinct_names(vec_str!["lack", "back"]), 0);
    }
}
