// https://leetcode.com/problems/find-indices-with-index-and-value-difference-ii/
// 2905. Find Indices With Equal Value
pub struct Solution;
impl Solution {
    pub fn find_indices(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32> {
        let index_difference = index_difference as usize;
        let mut min = i32::MAX;
        let mut imin = 0;
        let mut max = i32::MIN;
        let mut imax = 0;
        for i in index_difference..nums.len() {
            if nums[i - index_difference] < min {
                min = nums[i - index_difference];
                imin = i - index_difference;
            }
            if nums[i - index_difference] > max {
                max = nums[i - index_difference];
                imax = i - index_difference;
            }
            if (nums[i] - min).abs() >= value_difference {
                return vec![imin as i32, i as i32];
            }
            if (nums[i] - max).abs() >= value_difference {
                return vec![imax as i32, i as i32];
            }
        }
        vec![-1, -1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_indices() {
        fn check(nums: Vec<i32>, index_difference: i32, value_difference: i32, expected: Vec<i32>) {
            let actual = Solution::find_indices(nums.clone(), index_difference, value_difference);
            if expected == vec![-1, -1] {
                assert_eq!(actual, expected);
            } else {
                assert!(actual[0] >= 0 && actual[0] < nums.len() as i32);
                assert!(actual[1] >= 0 && actual[1] < nums.len() as i32);
                assert!((actual[0] - actual[1]).abs() >= index_difference);
                assert!((nums[actual[0] as usize] - nums[actual[1] as usize]).abs() >= value_difference);
            }
        }
        check(vec![5, 1, 4, 1], 2, 4, vec![0, 3]);
        check(vec![2, 1], 0, 0, vec![0, 0]);
        check(vec![1, 2, 3], 2, 4, vec![-1, -1]);
    }
}
