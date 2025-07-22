// https://leetcode.com/problems/sort-array-by-parity/
// 905. Sort Array By Parity
pub struct Solution;
impl Solution {
    pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_by_key(|x| x % 2);
        nums
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_array_by_parity() {
        assert_eq!(
            Solution::sort_array_by_parity(vec![3, 1, 2, 4]),
            vec![2, 4, 3, 1]
        );
        assert_eq!(Solution::sort_array_by_parity(vec![0]), vec![0]);
    }
}
