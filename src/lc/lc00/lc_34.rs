// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/
// 34. Find First and Last Position of Element in Sorted Array
pub struct Solution;
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let low = nums.partition_point(|x| x < &target);
        let high = nums.partition_point(|x| x <= &target);
        if low == high {
            vec![-1, -1]
        } else {
            vec![low as i32, high as i32 - 1]
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_search_range() {
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
            vec![3, 4]
        );
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
            vec![-1, -1]
        );
        assert_eq!(Solution::search_range(vec![], 0), vec![-1, -1]);
    }
}
