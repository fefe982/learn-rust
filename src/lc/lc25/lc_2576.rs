// https://leetcode.com/problems/find-the-maximum-number-of-marked-indices/
// 2576. Find the Maximum Number of Marked Indices
pub struct Solution;
impl Solution {
    pub fn max_num_of_marked_indices(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let len = nums.len();
        let mut h = len / 2;
        let mut sum = 0;
        for l in 0..len / 2 {
            while h < len && 2 * nums[l] > nums[h] {
                h += 1;
            }
            if h < len {
                sum += 2;
                h += 1;
            }
            if h >= len {
                break;
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_num_of_marked_indices() {
        assert_eq!(Solution::max_num_of_marked_indices(vec![3, 5, 2, 4]), 2);
        assert_eq!(Solution::max_num_of_marked_indices(vec![9, 2, 5, 4]), 4);
        assert_eq!(Solution::max_num_of_marked_indices(vec![7, 6, 8]), 0);
    }
}
