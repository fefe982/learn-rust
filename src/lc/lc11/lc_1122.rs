// https://leetcode.com/problems/relative-sort-array/
// 1122. Relative Sort Array
pub struct Solution;
impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut cnt = std::collections::HashMap::new();
        for &i in &arr2 {
            cnt.insert(i, 0);
        }
        let mut rest = vec![];
        for &i in &arr1 {
            if let Some(c) = cnt.get_mut(&i) {
                *c += 1;
            } else {
                rest.push(i);
            }
        }
        rest.sort_unstable();
        arr2.into_iter()
            .map(|i| std::iter::repeat(i).take(cnt.get(&i).cloned().unwrap() as usize))
            .flatten()
            .chain(rest)
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_relative_sort_array() {
        assert_eq!(
            Solution::relative_sort_array(vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19], vec![2, 1, 4, 3, 9, 6],),
            vec![2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19]
        );
        assert_eq!(
            Solution::relative_sort_array(vec![28, 6, 22, 8, 44, 17], vec![22, 28, 8, 6]),
            vec![22, 28, 8, 6, 17, 44]
        );
    }
}
