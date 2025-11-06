// https://leetcode.com/problems/shuffle-string/
// 1528. Shuffle String
pub struct Solution;
impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let mut si = s.chars().zip(indices).collect::<Vec<_>>();
        si.sort_by(|a, b| a.1.cmp(&b.1));
        si.into_iter().map(|x| x.0).collect::<String>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn restore_string() {
        assert_eq!(
            Solution::restore_string("codeleet".to_string(), vec![4, 5, 6, 7, 0, 2, 1, 3]),
            "leetcode".to_string()
        );
        assert_eq!(
            Solution::restore_string("abc".to_string(), vec![0, 1, 2]),
            "abc".to_string()
        );
    }
}
