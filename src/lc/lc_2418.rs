// https://leetcode.com/problems/sort-the-people/
// 2418. Sort the People
pub struct Solution;
impl Solution {
    pub fn sort_people(names: Vec<String>, mut heights: Vec<i32>) -> Vec<String> {
        for (idx, h) in heights.iter_mut().enumerate() {
            *h = *h * 1000 + idx as i32;
        }
        heights.sort_unstable_by_key(|x| -*x);
        heights
            .iter()
            .map(|&x| names[x as usize % 1000].clone())
            .collect()
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
