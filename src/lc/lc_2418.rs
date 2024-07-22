// https://leetcode.com/problems/sort-the-people/
// 2418. Sort the People
pub struct Solution;
impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut zip = names.into_iter().zip(heights).collect::<Vec<_>>();
        zip.sort_unstable_by_key(|x| -x.1);
        zip.into_iter().map(|x| x.0).collect::<Vec<_>>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_str;
    #[test]
    fn sort_people() {
        assert_eq!(
            Solution::sort_people(vec_str!["Mary", "John", "Emma"], vec![180, 165, 170]),
            vec_str!["Mary", "Emma", "John"]
        );
        assert_eq!(
            Solution::sort_people(vec_str!["Alice", "Bob", "Bob"], vec![155, 185, 150]),
            vec_str!["Bob", "Alice", "Bob"]
        );
    }
}
