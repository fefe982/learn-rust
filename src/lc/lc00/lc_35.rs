// https://leetcode.cn/problems/search-insert-position/
// 35. Search Insert Position
pub struct Solution;
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        nums.partition_point(|&x| x < target) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_search_insert() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 0), 0);
    }
}
