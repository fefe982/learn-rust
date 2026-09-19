// https://leetcode.com/problems/find-mirror-score-of-a-string/
// 3412. Find the Mirror Score of a String
pub struct Solution;
impl Solution {
    pub fn calculate_score(s: String) -> i64 {
        let mut m: std::collections::HashMap<usize, Vec<usize>> = std::collections::HashMap::new();
        let mut ans = 0;
        'c: for (i, c) in s.chars().enumerate() {
            let ic = c as usize - 'a' as usize;
            let im = 25 - ic;
            if let Some(v) = m.get_mut(&im) {
                if let Some(j) = v.pop() {
                    ans += (i - j) as i64;
                    continue 'c;
                }
            }
            m.entry(ic).or_insert(vec![]).push(i);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn calculate_score() {
        assert_eq!(Solution::calculate_score("aczzx".to_string()), 5);
        assert_eq!(Solution::calculate_score("abcdef".to_string()), 0);
    }
}
