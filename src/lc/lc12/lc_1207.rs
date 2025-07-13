// https://leetcode.com/problems/unique-number-of-occurrences/
// 1207. Unique Number of Occurrences
pub struct Solution;
impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut occ = std::collections::HashMap::<i32, i32>::new();
        for x in arr {
            *occ.entry(x).or_default() += 1;
        }
        let mut unq = std::collections::HashSet::<i32>::new();
        for v in occ.values() {
            if unq.contains(v) {
                return false;
            }
            unq.insert(*v);
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_unique_occurrences() {
        assert_eq!(Solution::unique_occurrences(vec![1, 2, 2, 1, 1, 3]), true);
        assert_eq!(Solution::unique_occurrences(vec![1, 2]), false);
        assert_eq!(
            Solution::unique_occurrences(vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0]),
            true
        );
    }
}
