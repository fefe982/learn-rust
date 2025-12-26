// https://leetcode.com/problems/find-indices-with-index-and-value-difference-i/
// 2903. Find Indices With Index and Value Difference I
pub struct Solution;
impl Solution {
    pub fn find_indices(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32> {
        let mut min = nums[0];
        let mut max = nums[0];
        let mut min_i = 0;
        let mut max_i = 0;
        for j in index_difference as usize..nums.len() {
            if (min - nums[j]).abs() >= value_difference {
                return vec![min_i, j as i32];
            }
            if (max - nums[j]).abs() >= value_difference {
                return vec![max_i, j as i32];
            }
            let ni = j - index_difference as usize + 1;
            if ni >= nums.len() {
                break;
            }
            if nums[ni] < min {
                min = nums[ni];
                min_i = ni as i32;
            }
            if nums[ni] > max {
                max = nums[ni];
                max_i = ni as i32;
            }
        }
        vec![-1, -1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn check(nums: &[i32], index_difference: i32, value_difference: i32, expected: bool) {
        let actual = Solution::find_indices(nums.to_vec(), index_difference, value_difference);
        if expected {
            assert!((actual[0] - actual[1]).abs() >= index_difference);
            assert!((nums[actual[0] as usize] - nums[actual[1] as usize]).abs() >= value_difference);
        } else {
            assert_eq!(actual, [-1, -1]);
        }
    }
    #[test]
    fn test_find_indices() {
        check(&[5, 1, 4, 1], 2, 4, true);
        check(&[2, 1], 0, 0, true);
        check(&[1, 2, 3], 2, 4, false);
    }
}
