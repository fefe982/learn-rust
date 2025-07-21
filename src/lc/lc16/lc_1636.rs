// https://leetcode.com/problems/sort-array-by-increasing-frequency/
// 1636. Sort Array by Increasing Frequency
pub struct Solution;
impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        let cnt = nums
            .into_iter()
            .fold(std::collections::HashMap::<i32, i32>::new(), |mut cnt, x| {
                *cnt.entry(x).or_default() += 1;
                cnt
            });
        let mut v = cnt.into_iter().collect::<Vec<_>>();
        v.sort_by_key(|v| (v.1, std::cmp::Reverse(v.0)));
        v.into_iter()
            .flat_map(|(n, c)| std::iter::repeat(n).take(c as usize))
            .collect::<Vec<_>>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_frequency_sort() {
        assert_eq!(Solution::frequency_sort(vec![1, 1, 2, 2, 2, 3]), vec![3, 1, 1, 2, 2, 2]);
        assert_eq!(Solution::frequency_sort(vec![2, 3, 1, 3, 2]), [1, 3, 3, 2, 2]);
        assert_eq!(
            Solution::frequency_sort(vec![-1, 1, -6, 4, 5, -6, 1, 4, 1]),
            [5, -1, 4, 4, -6, -6, 1, 1, 1]
        );
    }
}
